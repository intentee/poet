use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use rhai::Engine;
use rhai::module_resolvers::FileModuleResolver;
use rhai_components::builds_engine::BuildsEngine;
use rhai_components::component_syntax::component_reference_stub::ComponentReferenceStub;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

use crate::asset_manager::AssetManager;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_hierarchy::ContentDocumentHierarchy;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_tree_node::ContentDocumentTreeNode;
use crate::filesystem::file_entry::FileEntry;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;
use crate::rhai_helpers::render_hierarchy;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

pub struct RhaiTemplateRendererFactory {
    base_directory: PathBuf,
    component_registry: Arc<ComponentRegistry>,
    shortcodes_subdirectory: PathBuf,
}

impl RhaiTemplateRendererFactory {
    pub fn new(base_directory: PathBuf, shortcodes_subdirectory: PathBuf) -> Self {
        Self {
            base_directory,
            component_registry: Default::default(),
            shortcodes_subdirectory,
        }
    }

    pub fn register_component_file(&self, file_entry: FileEntry) {
        let component_name = file_entry.get_stem_relative_to(&self.shortcodes_subdirectory);

        self.component_registry
            .register_component_from_stub(ComponentReferenceStub {
                name: component_name.clone(),
                path: component_name,
            });
    }
}

impl BuildsEngine for RhaiTemplateRendererFactory {
    fn component_registry(&self) -> Arc<ComponentRegistry> {
        self.component_registry.clone()
    }

    fn prepare_engine(&self, engine: &mut Engine) -> Result<()> {
        engine.set_module_resolver(FileModuleResolver::new_with_path(
            self.base_directory.join(&self.shortcodes_subdirectory),
        ));

        engine.build_type::<ArgumentWithInput>();
        engine.build_type::<AssetManager>();
        engine.build_type::<ContentDocumentCollectionRanked>();
        engine.build_type::<ContentDocumentComponentContext>();
        engine.build_type::<ContentDocumentFrontMatter>();
        engine.build_type::<ContentDocumentHierarchy>();
        engine.build_type::<ContentDocumentReference>();
        engine.build_type::<ContentDocumentTreeNode>();
        engine.build_type::<FileEntry>();
        engine.build_type::<Heading>();
        engine.build_type::<PromptDocumentComponentContext>();
        engine.build_type::<PromptDocumentFrontMatter>();
        engine.build_type::<TableOfContents>();

        engine.register_fn("render_hierarchy", render_hierarchy);

        Ok(())
    }
}

impl TryInto<RhaiTemplateRenderer> for RhaiTemplateRendererFactory {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RhaiTemplateRenderer, Self::Error> {
        let expression_engine = self.create_engine()?;

        RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: self.component_registry,
            expression_engine,
        })
    }
}
