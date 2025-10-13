use schemars::JsonSchema;
use schemars::schema_for;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::mcp::tool::Tool;

pub trait ToolProvider: Send + Sync {
    type InputSchema: DeserializeOwned + JsonSchema + Serialize;
    type OutputSchema: DeserializeOwned + JsonSchema + Serialize;

    fn name(&self) -> String;

    fn description(&self) -> Option<String> {
        None
    }

    fn title(&self) -> Option<String> {
        None
    }

    fn tool_definition(&self) -> Tool {
        Tool {
            description: self.description(),
            input_schema: schema_for!(Self::InputSchema),
            name: self.name(),
            output_schema: schema_for!(Self::OutputSchema),
            title: self.title(),
        }
    }
}
