use std::path::Path;

use anyhow::Result;
use markdown::ParseOptions;
use markdown::Options;
use markdown::CompileOptions;
use markdown::Constructs;
use markdown::to_mdast;
use anyhow::anyhow;
use markdown::mdast::Node;
use markdown::MdxExpressionKind;
use markdown::MdxSignal;
use log::info;

use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;

fn parse_esm(contents: &str) -> MdxSignal {
    log::info!("Parsing ESM: {}", contents);

    MdxSignal::Ok
}

fn parse_expression(contents: &str, expression_kind: &MdxExpressionKind) -> MdxSignal {
    log::info!("Parsing ESM expression: {}", contents);

    MdxSignal::Ok
}

fn parse_file_contents(contents: &str) -> Result<Node> {
    match to_mdast(contents, &ParseOptions {
        // constructs: Constructs {
        //     frontmatter: true,
        //     ..Constructs::default()
        // },
        constructs: Constructs {
            autolink: true,
            frontmatter: true,
            html_flow: false,
            // html_text: false,
            mdx_jsx_flow: true,
            // mdx_jsx_text: true,
            ..Constructs::gfm()
        },
        // mdx_esm_parse: Some(Box::new(parse_esm)),
        // mdx_expression_parse: Some(Box::new(parse_expression)),
        ..ParseOptions::default()
    }) {
        Ok(node) => Ok(node),
        Err(message) => {
            Err(anyhow!("Failed to parse file contents: {message:?}"))
        }
    }
}

pub async fn build_project<TFilesystem>(source_filesystem: &TFilesystem) -> Result<Memory>
where
    TFilesystem: Filesystem,
{
    let files = source_filesystem.read_all_files().await?;

    for file in files {
        let ast = parse_file_contents(&file.contents)?;
        info!("Processing file: {:#?}, {:#?}", file.path, ast);
    }

    Err(anyhow!("Not implemented yet"))
}
