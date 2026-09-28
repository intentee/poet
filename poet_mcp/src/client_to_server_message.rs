use serde::Deserialize;
use serde::Serialize;

use crate::initialize_request::InitializeRequest;
use crate::initialized_notification::InitializedNotification;
use crate::logging_set_level_request::LoggingSetLevelRequest;
use crate::ping_request::PingRequest;
use crate::prompts_get_request::PromptsGetRequest;
use crate::prompts_list_request::PromptsListRequest;
use crate::resources_list_request::ResourcesListRequest;
use crate::resources_read_request::ResourcesReadRequest;
use crate::resources_subscribe_request::ResourcesSubscribeRequest;
use crate::resources_templates_list_request::ResourcesTemplatesListRequest;
use crate::resources_unsubscribe_request::ResourcesUnsubscribeRequest;
use crate::tools_call_request::ToolsCallRequest;
use crate::tools_list_request::ToolsListRequest;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "method")]
pub enum ClientToServerMessage {
    #[serde(rename = "initialize")]
    Initialize(InitializeRequest),
    #[serde(rename = "notifications/initialized")]
    Initialized(InitializedNotification),
    #[serde(rename = "logging/setLevel")]
    LoggingSetLevel(LoggingSetLevelRequest),
    #[serde(rename = "ping")]
    Ping(PingRequest),
    #[serde(rename = "prompts/get")]
    PromptsGet(PromptsGetRequest),
    #[serde(rename = "prompts/list")]
    PromptsList(PromptsListRequest),
    #[serde(rename = "resources/list")]
    ResourcesList(ResourcesListRequest),
    #[serde(rename = "resources/read")]
    ResourcesRead(ResourcesReadRequest),
    #[serde(rename = "resources/subscribe")]
    ResourcesSubscribe(ResourcesSubscribeRequest),
    #[serde(rename = "resources/templates/list")]
    ResourcesTemplatesList(ResourcesTemplatesListRequest),
    #[serde(rename = "resources/unsubscribe")]
    ResourcesUnsubscribe(ResourcesUnsubscribeRequest),
    #[serde(rename = "tools/call")]
    ToolsCall(ToolsCallRequest),
    #[serde(rename = "tools/list")]
    ToolsList(ToolsListRequest),
}
