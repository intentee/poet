use std::sync::Arc;

use markdown::mdast::Node;
use poet_mcp::prompt::Prompt;
use poet_mcp::prompt_argument::PromptArgument;
use poet_mcp::prompts_get_request::PromptsGetRequest;
use poet_mcp::prompts_get_request_params::PromptsGetRequestParams;
use poet_mcp::prompts_get_result::PromptsGetResult;

use crate::prompt_document_argument::PromptDocumentArgument;
use crate::prompt_document_evaluator::PromptDocumentEvaluator;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_error::PromptError;
use crate::prompt_rendering_context::PromptRenderingContext;

pub struct PromptDocumentController {
    pub front_matter: PromptDocumentFrontMatter,
    pub mdast: Node,
    pub name: String,
    pub rendering_context: Arc<PromptRenderingContext>,
}

impl PromptDocumentController {
    #[must_use]
    pub fn mcp_prompt(&self) -> Prompt {
        Prompt {
            arguments: self
                .front_matter
                .arguments
                .iter()
                .map(
                    |(
                        name,
                        PromptDocumentArgument {
                            description,
                            required,
                            title,
                        },
                    )| PromptArgument {
                        description: description.clone(),
                        name: name.clone(),
                        required: *required,
                        title: title.clone(),
                    },
                )
                .collect(),
            description: self.front_matter.description.clone(),
            name: self.name.clone(),
            title: self.front_matter.title.clone(),
        }
    }

    pub fn respond_to(
        &self,
        PromptsGetRequest {
            params: PromptsGetRequestParams { arguments, .. },
            ..
        }: PromptsGetRequest,
    ) -> Result<PromptsGetResult, PromptError> {
        self.front_matter
            .map_arguments(&arguments)
            .map(|arguments| {
                self.rendering_context
                    .component_context(arguments, self.front_matter.clone())
            })
            .and_then(|component_context| {
                PromptDocumentEvaluator {
                    component_context: &component_context,
                    rhai_template_renderer: &self.rendering_context.rhai_template_renderer,
                }
                .assemble_messages(&self.mdast)
            })
            .map(|messages| PromptsGetResult {
                description: Some(self.front_matter.description.clone()),
                messages,
                meta: None,
            })
    }
}
