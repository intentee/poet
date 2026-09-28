use std::path::PathBuf;
use std::sync::Arc;

use poet_filesystem::file_entry::FileEntry;
use poet_filesystem::filesystem_error::FilesystemError;
use rhai::Engine;
use rhai::module_resolvers::FileModuleResolver;
use rhai_components::component_syntax::component_reference::ComponentReference;
use rhai_components::component_syntax::component_registry::ComponentRegistry;
use rhai_components::create_component_engine::create_component_engine;
use rhai_components::rhai_components_error::RhaiComponentsError;
use rhai_components::rhai_template_renderer::RhaiTemplateRenderer;
use rhai_components::rhai_template_renderer_params::RhaiTemplateRendererParams;

use crate::asset_manager::AssetManager;
use crate::author::Author;
use crate::author_collection::AuthorCollection;
use crate::author_data::AuthorData;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_component_context::ContentDocumentComponentContext;
use crate::content_document_front_matter::ContentDocumentFrontMatter;
use crate::content_document_hierarchy::ContentDocumentHierarchy;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_tree_node::ContentDocumentTreeNode;
use crate::prompt_document_component_context::PromptDocumentComponentContext;
use crate::prompt_document_front_matter::PromptDocumentFrontMatter;
use crate::prompt_document_front_matter::argument_with_input::ArgumentWithInput;
use crate::rhai_helpers::render_hierarchy;
use crate::shortcodes_source_directory::SHORTCODES_SOURCE_DIRECTORY;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

pub struct RhaiTemplateRendererFactory {
    base_directory: PathBuf,
    component_registry: Arc<ComponentRegistry>,
}

impl RhaiTemplateRendererFactory {
    pub fn new(base_directory: PathBuf) -> Self {
        Self {
            base_directory,
            component_registry: Default::default(),
        }
    }

    pub fn register_component_file(&self, file_entry: &FileEntry) -> Result<(), FilesystemError> {
        self.component_registry
            .register_component(ComponentReference {
                name: file_entry.stem_in(&SHORTCODES_SOURCE_DIRECTORY)?,
            });

        Ok(())
    }
}

impl RhaiTemplateRendererFactory {
    fn create_engine(&self) -> Engine {
        let mut engine = create_component_engine();

        engine.set_module_resolver(FileModuleResolver::new_with_path(
            self.base_directory.join(SHORTCODES_SOURCE_DIRECTORY.name),
        ));

        engine.build_type::<ArgumentWithInput>();
        engine.build_type::<AssetManager>();
        engine.build_type::<Author>();
        engine.build_type::<AuthorCollection>();
        engine.build_type::<AuthorData>();
        engine.build_type::<ContentDocumentCollectionRanked>();
        engine.build_type::<ContentDocumentComponentContext>();
        engine.build_type::<ContentDocumentFrontMatter>();
        engine.build_type::<ContentDocumentHierarchy>();
        engine.build_type::<ContentDocumentReference>();
        engine.build_type::<ContentDocumentTreeNode>();
        engine.build_type::<Heading>();
        engine.build_type::<PromptDocumentComponentContext>();
        engine.build_type::<PromptDocumentFrontMatter>();
        engine.build_type::<TableOfContents>();

        engine.register_fn("render_hierarchy", render_hierarchy);

        engine
    }
}

impl TryInto<RhaiTemplateRenderer> for RhaiTemplateRendererFactory {
    type Error = RhaiComponentsError;

    fn try_into(self) -> Result<RhaiTemplateRenderer, Self::Error> {
        let expression_engine = self.create_engine();

        RhaiTemplateRenderer::build(RhaiTemplateRendererParams {
            component_registry: self.component_registry,
            expression_engine,
        })
    }
}
