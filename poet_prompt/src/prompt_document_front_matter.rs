use std::collections::BTreeMap;
use std::collections::HashMap;

use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;

use crate::prompt_document_argument::PromptDocumentArgument;
use crate::prompt_document_argument_with_input::PromptDocumentArgumentWithInput;
use crate::prompt_error::PromptError;

#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromptDocumentFrontMatter {
    pub arguments: BTreeMap<String, PromptDocumentArgument>,
    pub description: String,
    pub title: String,
}

impl PromptDocumentFrontMatter {
    pub fn map_arguments(
        &self,
        inputs: &HashMap<String, String>,
    ) -> Result<BTreeMap<String, PromptDocumentArgumentWithInput>, PromptError> {
        self.arguments
            .iter()
            .map(
                |(
                    name,
                    PromptDocumentArgument {
                        description,
                        required,
                        title,
                    },
                )| {
                    inputs
                        .get(name)
                        .map(|input| {
                            (
                                name.clone(),
                                PromptDocumentArgumentWithInput {
                                    description: description.clone(),
                                    input: input.clone(),
                                    required: *required,
                                    title: title.clone(),
                                },
                            )
                        })
                        .ok_or_else(|| PromptError::MissingArgument { name: name.clone() })
                },
            )
            .collect()
    }

    fn rhai_description(&mut self) -> String {
        self.description.clone()
    }

    fn rhai_title(&mut self) -> String {
        self.title.clone()
    }
}

impl CustomType for PromptDocumentFrontMatter {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("PromptDocumentFrontMatter")
            .with_get("description", Self::rhai_description)
            .with_get("title", Self::rhai_title);
    }
}
