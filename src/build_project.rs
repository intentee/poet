use std::path::Path;

use anyhow::Result;
use anyhow::anyhow;
use log::info;
use markdown::CompileOptions;
use markdown::Constructs;
use markdown::MdxExpressionKind;
use markdown::MdxSignal;
use markdown::Options;
use markdown::ParseOptions;
use markdown::mdast::Node;
use markdown::to_mdast;
use rhai::Engine;
use rhai::EvalAltResult;
use rhai::module_resolvers::FileModuleResolver;

use crate::filesystem::Filesystem;
use crate::filesystem::memory::Memory;
use crate::filesystem::storage::Storage;
use crate::rhai_context::RhaiContext;

fn parse_esm(contents: &str) -> MdxSignal {
    log::info!("Parsing ESM: {}", contents);

    MdxSignal::Ok
}

fn parse_expression(contents: &str, expression_kind: &MdxExpressionKind) -> MdxSignal {
    log::info!("Parsing ESM expression: {}", contents);

    MdxSignal::Ok
}

fn parse_file_contents(contents: &str) -> Result<Node> {
    match to_mdast(
        contents,
        &ParseOptions {
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
        },
    ) {
        Ok(node) => Ok(node),
        Err(message) => Err(anyhow!("Failed to parse file contents: {message:?}")),
    }
}

pub async fn build_project(source_filesystem: &Storage) -> Result<Memory> {
    let files = source_filesystem.read_project_files().await?;
    let rhai_context = RhaiContext::new(source_filesystem.base_directory.join("shortcodes"));

    // First pass, process Rhai files to be used as shortcodes or layouts
    for file in &files {
        if file.is_rhai() {
            info!("Processing Rhai file: {:?}", file.path);

            rhai_context.compile_template_file(&file)?;
        }
    }

    for file in &files {
        if file.is_markdown() {
            // let ast = parse_file_contents(&file.contents)?;
            info!("Processing file: {:?}", file.path);
        }
    }

    Err(anyhow!("Not implemented yet"))
}
