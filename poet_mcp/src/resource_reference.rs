use http::Uri;

use crate::mcp_error::McpError;

#[derive(Clone, Debug)]
pub struct ResourceReference {
    pub class: String,
    pub path: String,
    pub scheme: String,
    pub uri_string: String,
}

impl TryFrom<&str> for ResourceReference {
    type Error = McpError;

    fn try_from(uri_string: &str) -> Result<Self, Self::Error> {
        let uri: Uri = uri_string
            .parse()
            .map_err(|source| McpError::InvalidResourceUri {
                uri: uri_string.to_owned(),
                source,
            })?;
        let class = uri
            .authority()
            .ok_or_else(|| McpError::MissingResourceUriAuthority {
                uri: uri_string.to_owned(),
            })?
            .host()
            .to_owned();
        let scheme = uri
            .scheme_str()
            .ok_or_else(|| McpError::MissingResourceUriScheme {
                uri: uri_string.to_owned(),
            })?
            .to_owned();
        let path = uri.path();

        Ok(Self {
            class,
            path: path.strip_prefix('/').unwrap_or(path).to_owned(),
            scheme,
            uri_string: uri.to_string(),
        })
    }
}
