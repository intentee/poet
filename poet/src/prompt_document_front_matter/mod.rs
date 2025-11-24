pub mod argument;
pub mod argument_with_input;

use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;
use rhai::CustomType;
use rhai::TypeBuilder;
use serde::Deserialize;
use serde::Serialize;

use self::argument::Argument;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PromptDocumentFrontMatter {
    pub arguments: HashMap<String, Argument>,
    pub description: String,
    pub title: String,
}

impl PromptDocumentFrontMatter {
    pub fn map_arguments(
        &self,
        inputs: HashMap<String, String>,
    ) -> Result<HashMap<String, ArgumentWithInput>> {
        self.arguments
            .clone()
            .into_iter()
            .map(
                |(
                    name,
                    Argument {
                        description,
                        required,
                        title,
                    },
                )| {
                    Ok((
                        name.clone(),
                        ArgumentWithInput {
                            description,
                            input: inputs
                                .get(&name)
                                .ok_or_else(|| anyhow!("No argument provided for '{name}'"))?
                                .to_string(),
                            required,
                            title,
                        },
                    ))
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
