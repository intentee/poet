mod app_data;
mod http_route;

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use actix_files::Files;
use actix_web::App;
use actix_web::HttpServer;
use actix_web::web::Data;
use anyhow::Result;
use async_trait::async_trait;
use clap::Parser;
use indoc::formatdoc;
use log::info;
use poet_assets::asset_path_renderer::AssetPathRenderer;
use poet_assets::read_esbuild_metafile_or_default::read_esbuild_metafile_or_default;
use poet_content::build_authors::build_authors;
use poet_content::build_project::build_project;
use poet_content::build_project::build_project_params::BuildProjectParams;
use poet_content::build_project::build_project_result::BuildProjectResult;
use poet_filesystem::filesystem::Filesystem;
use poet_mcp::implementation::Implementation;
use poet_mcp::mcp_http_service_factory::McpHttpServiceFactory;

use crate::compile_poet_shortcodes::compile_poet_shortcodes;
use crate::app_dir_desktop_entry::AppDirDesktopEntry;
use crate::build_project_result_holder::BuildProjectResultHolder;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection;
use crate::build_prompt_document_controller_collection::build_prompt_document_controller_collection_params::BuildPromptControllerCollectionParams;
use crate::holder::Holder as _;
use crate::cmd::MCP_STREAMABLE_HTTP_MOUNT_PATH;
use crate::cmd::STATIC_FILES_PUBLIC_PATH;
use crate::cmd::builds_project::BuildsProject;
use crate::cmd::handler::Handler;
use crate::cmd::serve::app_data::AppData;
use crate::cmd::value_parser::parse_socket_addr;
use crate::cmd::value_parser::validate_is_directory;
use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::mcp_server_factory::McpServerFactory;
use crate::prompt_controller_collection_holder::PromptControllerCollectionHolder;
use crate::search_index::SearchIndex;
use crate::search_index_reader::SearchIndexReader;
use crate::search_index_reader_holder::SearchIndexReaderHolder;

#[derive(Parser)]
pub struct Serve {
    #[arg(long, default_value="127.0.0.1:8070", value_parser = parse_socket_addr)]
    addr: SocketAddr,

    #[arg(value_parser = validate_is_directory)]
    app_dir: PathBuf,

    #[arg(long)]
    app_name: String,

    #[arg(long)]
    public_path: String,

    #[arg(long, default_value = "false")]
    sitemap: bool,
}

impl BuildsProject for Serve {
    fn source_directory(&self) -> PathBuf {
        self.app_dir.clone()
    }
}

#[async_trait(?Send)]
impl Handler for Serve {
    async fn handle(&self) -> Result<()> {
        let asset_path_renderer = AssetPathRenderer {
            base_path: self.public_path.clone(),
        };
        let source_filesystem = self.source_filesystem();
        let rhai_template_renderer = compile_poet_shortcodes(&source_filesystem).await?;
        let app_dir_desktop_entry = AppDirDesktopEntry::parse(
            &source_filesystem
                .read_file_contents_string(&PathBuf::from(format!(
                    "{}.desktop",
                    self.app_name.to_lowercase()
                )))
                .await?,
        )?;

        info!(
            "{}",
            formatdoc! {
                r#"
                    Site details:
                    ├── name: {name}
                    ├── title: {title}
                    ├── generated with Poet version: {poet_version}
                    └── version: {site_version}
                "#,
                name = app_dir_desktop_entry.name,
                poet_version = app_dir_desktop_entry.poet_version,
                site_version = app_dir_desktop_entry.site_version,
                title = app_dir_desktop_entry.title,
            }
        );

        let server_info = Implementation {
            description: None,
            name: app_dir_desktop_entry.name.clone(),
            title: Some(app_dir_desktop_entry.title.clone()),
            version: app_dir_desktop_entry.site_version.clone(),
        };

        let authors = build_authors(source_filesystem.clone()).await?;

        let build_project_result: BuildProjectResult = build_project(BuildProjectParams {
            asset_path_renderer: asset_path_renderer.clone(),
            authors,
            esbuild_metafile: read_esbuild_metafile_or_default(source_filesystem.as_ref()).await?,
            generated_page_base_path: self.public_path.clone(),
            generate_sitemap: self.sitemap,
            is_watching: false,
            rhai_template_renderer: rhai_template_renderer.clone(),
            source_filesystem: source_filesystem.clone(),
        })
        .await?
        .into();

        let prompt_controller_collection =
            build_prompt_document_controller_collection(BuildPromptControllerCollectionParams {
                asset_path_renderer: asset_path_renderer.clone(),
                content_document_linker: build_project_result.content_document_linker.clone(),
                esbuild_metafile: build_project_result.esbuild_metafile.clone(),
                rhai_template_renderer,
                source_filesystem: source_filesystem.clone(),
            })
            .await?;

        let prompt_controller_collection_holder: PromptControllerCollectionHolder =
            Default::default();

        prompt_controller_collection_holder
            .set(Some(Arc::new(prompt_controller_collection)))
            .await;

        let app_data = Data::new(AppData {
            filesystem_http_route_index: Arc::new(FilesystemHttpRouteIndex::from_memory(
                &build_project_result.memory_filesystem,
            )?),
        });

        let assets_directory = self.app_dir.join(STATIC_FILES_PUBLIC_PATH);
        let search_index_reader: SearchIndexReader =
            SearchIndex::create_in_memory(build_project_result.content_document_sources.clone())
                .index()?;
        let build_project_result_holder: BuildProjectResultHolder = Default::default();

        build_project_result_holder
            .set(Some(build_project_result))
            .await;

        let search_index_reader_holder: SearchIndexReaderHolder = Default::default();

        search_index_reader_holder
            .set(Some(Arc::new(search_index_reader)))
            .await;

        let mcp_server = McpServerFactory {
            build_project_result_holder,
            prompt_controller_collection_holder,
            search_index_reader_holder,
            server_info,
        }
        .create();

        HttpServer::new(move || {
            App::new()
                .app_data(app_data.clone())
                .service(
                    Files::new(STATIC_FILES_PUBLIC_PATH, assets_directory.clone())
                        .prefer_utf8(true),
                )
                .service(McpHttpServiceFactory {
                    mcp_server: mcp_server.clone(),
                    mount_path: MCP_STREAMABLE_HTTP_MOUNT_PATH.to_owned(),
                })
                .configure(http_route::generated_pages::register)
        })
        .bind(self.addr)
        .expect("Unable to bind server to address")
        .shutdown_timeout(1)
        .run()
        .await?;

        Ok(())
    }
}
