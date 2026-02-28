use serde::Deserialize;
use serde::Serialize;

fn default_render() -> bool {
    true
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BlogPostFrontMatter {
    #[serde(default)]
    pub authors: Vec<String>,
    pub description: String,
    pub layout: String,
    #[serde(default = "default_render")]
    pub render: bool,
    pub title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_defaults_to_true() {
        let input = r#"
            description = "A post"
            layout = "PageBlogPost"
            title = "Hello"
        "#;

        let front_matter: BlogPostFrontMatter = toml::from_str(input).unwrap();

        assert_eq!(front_matter.render, true);
    }
}
