use thiserror::Error;

#[derive(Debug, Error)]
pub enum LayeredError {
    #[error("unable to publish the site")]
    Publish(#[source] Box<Self>),
    #[error("unable to render page '{page}'")]
    Render {
        page: String,
        #[source]
        source: Box<Self>,
    },
    #[error("template '{template}' does not exist")]
    TemplateNotFound { template: String },
}
