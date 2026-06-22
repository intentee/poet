use anyhow::Result;
use anyhow::anyhow;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum Role {
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl TryFrom<&str> for Role {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "assistant" => Ok(Role::Assistant),
            "user" => Ok(Role::User),
            _ => Err(anyhow!("Unknown role: {value}")),
        }
    }
}

impl TryFrom<String> for Role {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self> {
        value.as_str().try_into()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn converts_known_role_strings() -> Result<()> {
        assert_eq!(Role::try_from("assistant")?, Role::Assistant);
        assert_eq!(Role::try_from("user")?, Role::User);

        Ok(())
    }

    #[test]
    fn rejects_unknown_role_string() {
        assert!(Role::try_from("system").is_err());
    }

    #[test]
    fn converts_from_owned_string() -> Result<()> {
        assert_eq!(Role::try_from("user".to_string())?, Role::User);

        Ok(())
    }
}
