use std::sync::Arc;

use anyhow::Result;
use esbuild_metafile::EsbuildMetaFile;
use markdown::mdast::Node;

use crate::asset_manager::AssetManager;
use crate::asset_path_renderer::AssetPathRenderer;
use crate::content_document_linker::ContentDocumentLinker;
use crate::eval_prompt_document_mdast::eval_prompt_document_mdast;
use crate::mcp::jsonrpc::request::prompts_get::PromptsGet;
use crate::mcp::jsonrpc::request::prompts_get::PromptsGetParams;
use crate::mcp::jsonrpc::response::success::prompts_get_result::PromptsGetResult;
use crate::mcp::prompt::Prompt;
use crate::mcp::prompt::PromptArgument;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument::Argument;
use crate::rhai_template_renderer::RhaiTemplateRenderer;

pub struct PromptController {
    pub asset_path_renderer: AssetPathRenderer,
    pub content_document_linker: ContentDocumentLinker,
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub front_matter: PromptDocumentFrontMatter,
    pub name: String,
    pub mdast: Node,
    pub rhai_template_renderer: RhaiTemplateRenderer,
}

impl PromptController {
    pub fn get_mcp_prompt(&self) -> Prompt {
        Prompt {
            arguments: self
                .front_matter
                .clone()
                .arguments
                .into_iter()
                .map(
                    |(
                        name,
                        Argument {
                            description,
                            required,
                            title,
                        },
                    )| PromptArgument {
                        description,
                        name,
                        required,
                        title,
                    },
                )
                .collect(),
            description: self.front_matter.description.clone(),
            name: self.name.clone(),
            title: self.front_matter.title.clone(),
        }
    }

    pub async fn respond_to(
        &self,
        PromptsGet {
            params: PromptsGetParams { arguments, .. },
            ..
        }: PromptsGet,
    ) -> Result<PromptsGetResult> {
        eval_prompt_document_mdast(
            &self.mdast,
            &PromptDocumentComponentContext {
                arguments: self.front_matter.map_arguments(arguments)?,
                asset_manager: AssetManager::from_esbuild_metafile(
                    self.esbuild_metafile.clone(),
                    self.asset_path_renderer.clone(),
                ),
                content_document_linker: self.content_document_linker.clone(),
                front_matter: self.front_matter.clone(),
            },
            &self.rhai_template_renderer,
        )?;

        Ok(PromptsGetResult {
            description: Some(self.front_matter.description.clone()),
            messages: vec![],
            meta: None,
        })
    }
}
