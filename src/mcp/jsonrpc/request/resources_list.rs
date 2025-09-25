use serde::Deserialize;
use serde::Serialize;

use crate::mcp::jsonrpc::id::Id;
use crate::mcp::jsonrpc::meta::Meta;
use crate::mcp::list_resources_cursor::ListResourcesCursor;

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesListParams {
    #[serde(default, with = "crate::mcp::list_resources_cursor")]
    pub cursor: Option<ListResourcesCursor>,
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResourcesList {
    pub id: Id,
    pub jsonrpc: String,
    pub params: ResourcesListParams,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;
    use crate::mcp::jsonrpc::JSONRPC_VERSION;

    #[test]
    fn test_serialize_none() -> Result<()> {
        let resources_list = ResourcesList {
            id: Id::Number(1),
            jsonrpc: JSONRPC_VERSION.to_string(),
            params: ResourcesListParams {
                cursor: None,
                meta: None,
            },
        };

        let serialized = serde_json::to_string(&resources_list)?;

        assert_eq!(
            serialized,
            r#"{"id":1,"jsonrpc":"2.0","params":{"cursor":null}}"#
        );

        Ok(())
    }

    #[test]
    fn test_serialize_cursor() -> Result<()> {
        let resources_list = ResourcesList {
            id: Id::Number(1),
            jsonrpc: JSONRPC_VERSION.to_string(),
            params: ResourcesListParams {
                cursor: Some(ListResourcesCursor { offset: 5 }),
                meta: None,
            },
        };

        let serialized = serde_json::to_string(&resources_list)?;
        let deserialized: ResourcesList = serde_json::from_str(&serialized)?;

        assert_eq!(5, deserialized.params.cursor.unwrap().offset);

        Ok(())
    }
}
