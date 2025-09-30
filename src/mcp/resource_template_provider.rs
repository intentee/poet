use crate::mcp::resource_reference::ResourceReference;
use crate::mcp::resource_template::ResourceTemplate;

pub trait ResourceTemplateProvider: Send + Sync {
    fn mime_type(&self) -> String;

    fn resource_class(&self) -> String;

    fn resource_scheme(&self) -> String;

    fn can_handle(&self, ResourceReference { class, scheme, .. }: &ResourceReference) -> bool {
        *class == self.resource_class() && *scheme == self.resource_scheme()
    }

    fn resource_template(&self) -> ResourceTemplate {
        ResourceTemplate {
            description: self.description(),
            mime_type: self.mime_type(),
            name: self.name(),
            title: self.title(),
            uri_template: self.uri_template(),
        }
    }

    fn resource_uri_prefix(&self) -> String {
        format!("{}://{}", self.resource_scheme(), self.resource_class())
    }

    fn description(&self) -> Option<String> {
        None
    }

    fn name(&self) -> String {
        self.resource_class()
    }

    fn title(&self) -> Option<String> {
        None
    }

    fn uri_template(&self) -> String {
        format!("{}/{{path}}", self.resource_uri_prefix())
    }
}
