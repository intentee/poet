use anyhow::Result;
use anyhow::anyhow;
use http::Uri;

#[derive(Clone, Debug)]
pub struct ResourceReference {
    pub class: String,
    pub path: String,
    pub scheme: String,
    pub uri_string: String,
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
            uri_string: uri.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_scheme_class_and_stripped_path() -> Result<()> {
        let reference = ResourceReference::try_from("res://documents/guide/intro".parse::<Uri>()?)?;

        assert_eq!(reference.scheme, "res");
        assert_eq!(reference.class, "documents");
        assert_eq!(reference.path, "guide/intro");
        assert_eq!(reference.uri_string, "res://documents/guide/intro");

        Ok(())
    }

    #[test]
    fn keeps_empty_path_when_no_path_segment() -> Result<()> {
        let reference = ResourceReference::try_from("res://documents".parse::<Uri>()?)?;

        assert_eq!(reference.class, "documents");
        assert_eq!(reference.path, "");

        Ok(())
    }

    #[test]
    fn fails_when_scheme_is_missing() -> Result<()> {
        assert!(ResourceReference::try_from("//documents/guide".parse::<Uri>()?).is_err());

        Ok(())
    }

    #[test]
    fn fails_when_authority_is_missing() -> Result<()> {
        assert!(ResourceReference::try_from("/guide".parse::<Uri>()?).is_err());

        Ok(())
    }
}
