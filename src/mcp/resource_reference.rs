use anyhow::Result;
use anyhow::anyhow;
use http::Uri;

#[derive(Debug)]
pub struct ResourceReference {
    pub class: String,
    pub path: String,
    pub scheme: String,
    pub uri: Uri,
}

impl TryFrom<Uri> for ResourceReference {
    type Error = anyhow::Error;

    fn try_from(uri: Uri) -> Result<Self> {
        let path = uri.path().to_string();
        let path_stripped = if path.starts_with("/") {
            path.strip_prefix("/")
                .ok_or_else(|| anyhow!("Unable to strip path prefix from '{path}'"))?
                .to_string()
        } else {
            path
        };

        Ok(Self {
            class: uri
                .authority()
                .ok_or_else(|| anyhow!("Unable to establish uri authority: {uri}"))?
                .host()
                .to_string(),
            path: path_stripped,
            scheme: uri
                .scheme_str()
                .ok_or_else(|| anyhow!("Unable to establish uri scheme: {uri}"))?
                .to_string(),
            uri,
        })
    }
}
