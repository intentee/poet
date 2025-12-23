pub mod builds_engine;
pub mod component_syntax;
pub mod escape_html;
pub mod escape_html_attribute;
pub mod rhai_call_template_function;
pub mod rhai_helpers;
pub mod rhai_template_renderer;
pub mod rhai_template_renderer_params;

pub type SmartStringLazy = smartstring::SmartString<smartstring::LazyCompact>;
