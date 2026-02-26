use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::Context as _;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use embedding_codec::EmbeddingCodec;
use log::info;
use paddler_types::embedding::Embedding;
use paddler_types::embedding_input_document::EmbeddingInputDocument;
use tokio::fs;
use url::Url;

use crate::build_content_document_sources::build_content_document_sources;
use crate::build_timer::BuildTimer;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::value_parser::parse_socket_addr;
use crate::cmd::value_parser::validate_is_directory;
use crate::find_text_content_in_mdast::find_text_content_in_mdast;
use crate::generate_embedding::paddler_embedding_client::PaddlerEmbeddingClient;

#[derive(Parser)]
pub struct Embeddings {
    #[arg(long, value_parser = parse_socket_addr)]
    paddler_addr: SocketAddr,

    #[arg(long)]
    output_file: PathBuf,

    #[arg(value_parser = validate_is_directory)]
    source_directory: PathBuf,
}

impl BuildsProject for Embeddings {
    fn source_directory(&self) -> PathBuf {
        self.source_directory.clone()
    }
}

#[async_trait(?Send)]
impl Handler for Embeddings {
    async fn handle(&self) -> Result<()> {
        let source_filesystem = self.source_filesystem();

        let content_document_sources = build_content_document_sources(&source_filesystem, "")
            .await?
            .content_document_sources;

        let documents: Vec<EmbeddingInputDocument> = content_document_sources
            .iter()
            .filter_map(|(basename, source)| {
                let body = find_text_content_in_mdast(&source.mdast).ok()?;
                let title = &source.reference.front_matter.title;
                let description = &source.reference.front_matter.description;

                if body.is_empty() {
                    return None;
                }

                Some(EmbeddingInputDocument {
                    id: basename.to_string(),
                    content: format!("{title}\n{description}"),
                })
            })
            .collect();

        info!("Generating embeddings for {} documents...", documents.len());

        let _build_timer = BuildTimer::default();
        let inference_url: Url = Url::from_str(&format!("http://{}", self.paddler_addr))?;
        let client = PaddlerEmbeddingClient::new(inference_url);
        let results: Vec<Embedding> = client.generate_embeddings(documents).await?;

        let embeddings_map: BTreeMap<String, Vec<f32>> = results
            .into_iter()
            .map(|embedding| (embedding.source_document_id, embedding.embedding))
            .collect();

        let encoded = EmbeddingCodec::serialize(&embeddings_map);

        info!(
            "Saving {} embeddings to {}...",
            embeddings_map.len(),
            self.output_file.display()
        );

        fs::write(&self.output_file, encoded)
            .await
            .context(format!(
                "Failed to write file: {}",
                self.output_file.display()
            ))?;

        info!("Done.");

        Ok(())
    }
}
