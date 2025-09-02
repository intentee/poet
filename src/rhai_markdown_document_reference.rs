use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::markdown_document_reference::MarkdownDocumentReference;
use crate::rhai_front_matter::RhaiFrontMatter;

#[derive(Clone, Debug)]
pub struct RhaiMarkdownDocumentReference {
    pub front_matter: RhaiFrontMatter,
    pub reference: MarkdownDocumentReference,
}

impl RhaiMarkdownDocumentReference {
    fn rhai_basename(&mut self) -> String {
        self.reference.basename()
    }

    fn rhai_canonical_link(&mut self) -> Result<String, Box<EvalAltResult>> {
        Ok(self.reference.canonical_link()?)
    }

    fn rhai_front_matter(&mut self) -> RhaiFrontMatter {
        self.front_matter.clone()
    }
}

impl CustomType for RhaiMarkdownDocumentReference {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiMarkdownDocumentReference")
            .with_get("basename", Self::rhai_basename)
            .with_get("canonical_link", Self::rhai_canonical_link)
            .with_get("front_matter", Self::rhai_front_matter);
    }
}
