use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::notification::initialized::Initialized;
use crate::mcp::jsonrpc::request::initialize::Initialize;
use crate::mcp::jsonrpc::request::logging_set_level::LoggingSetLevel;
use crate::mcp::jsonrpc::request::ping::Ping;
use crate::mcp::jsonrpc::request::prompts_list::PromptsList;
use crate::mcp::jsonrpc::request::resources_list::ResourcesList;
use crate::mcp::jsonrpc::request::resources_read::ResourcesRead;
use crate::mcp::jsonrpc::request::resources_subscribe::ResourcesSubscribe;
use crate::mcp::jsonrpc::request::resources_templates_list::ResourcesTemplatesList;
use crate::mcp::jsonrpc::request::resources_unsubscribe::ResourcesUnsubscribe;
use crate::mcp::jsonrpc::request::tools_list::ToolsList;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ClientToServerMessage {
    #[serde(rename = "initialize")]
    Initialize(Initialize),
    #[serde(rename = "notifications/initialized")]
    Initialized(Initialized),
    #[serde(rename = "logging/setLevel")]
    LoggingSetLevel(LoggingSetLevel),
    #[serde(rename = "ping")]
    Ping(Ping),
    #[serde(rename = "prompts/list")]
    PromptsList(PromptsList),
    #[serde(rename = "resources/list")]
    ResourcesList(ResourcesList),
    #[serde(rename = "resources/read")]
    ResourcesRead(ResourcesRead),
    #[serde(rename = "resources/subscribe")]
    ResourcesSubscribe(ResourcesSubscribe),
    #[serde(rename = "resources/templates/list")]
    ResourcesTemplatesList(ResourcesTemplatesList),
    #[serde(rename = "resources/unsubscribe")]
    ResourcesUnsubscribe(ResourcesUnsubscribe),
    #[serde(rename = "tools/list")]
    ToolsList(ToolsList),
}
