use crate::blog_config::BlogConfig;
use crate::blog_name::BlogName;

#[derive(Clone, Debug)]
pub struct Blog {
    pub name: BlogName,
    pub config: BlogConfig,
}
