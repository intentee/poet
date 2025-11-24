use markdown::mdast::Node;

use crate::rhai_template_renderer::RhaiTemplateRenderer;

#[derive(Clone)]
pub struct EvalPromptDocumentMdastParams<'eval> {
    pub mdast: &'eval Node,
    pub is_directly_in_root: bool,
    pub is_first_child: bool,
    pub is_in_top_paragraph: bool,
    pub rhai_template_renderer: &'eval RhaiTemplateRenderer,
}

impl<'eval> EvalPromptDocumentMdastParams<'eval> {
    pub fn child(&self, node: &'eval Node, is_first_child: bool) -> Self {
        Self {
            mdast: node,
            is_directly_in_root: self.is_directly_in_root,
            is_first_child,
            is_in_top_paragraph: self.is_in_top_paragraph,
            rhai_template_renderer: self.rhai_template_renderer,
        }
    }

    pub fn directly_in_root(self) -> Self {
        Self {
            mdast: self.mdast,
            is_directly_in_root: true,
            is_first_child: self.is_first_child,
            is_in_top_paragraph: false,
            rhai_template_renderer: self.rhai_template_renderer,
        }
    }

    pub fn paragraph(self) -> Self {
        Self {
            mdast: self.mdast,
            is_directly_in_root: false,
            is_first_child: self.is_first_child,
            is_in_top_paragraph: self.is_directly_in_root,
            rhai_template_renderer: self.rhai_template_renderer,
        }
    }

    pub fn regular_element(self) -> Self {
        Self {
            mdast: self.mdast,
            is_directly_in_root: false,
            is_first_child: false,
            is_in_top_paragraph: false,
            rhai_template_renderer: self.rhai_template_renderer,
        }
    }
}
