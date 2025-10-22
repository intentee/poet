use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use dashmap::DashMap;
use rhai::Engine;
use rhai::Position;
use rhai::module_resolvers::FileModuleResolver;

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
use crate::rhai_components::component_meta_module::ComponentMetaModule;
use crate::rhai_components::component_reference::ComponentReference;
use crate::rhai_components::component_registry::ComponentRegistry;
use crate::rhai_components::evaluator_factory::EvaluatorFactory;
use crate::rhai_components::parse_component::parse_component;
use crate::rhai_functions::clsx;
use crate::rhai_functions::error;
use crate::rhai_functions::has;
use crate::rhai_functions::render_hierarchy;
use crate::rhai_safe_random_affix::rhai_safe_random_affix;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::table_of_contents::TableOfContents;
use crate::table_of_contents::heading::Heading;

pub struct RhaiTemplateFactory {
    base_directory: PathBuf,
    component_registry: Arc<ComponentRegistry>,
    shortcodes_subdirectory: PathBuf,
}

impl RhaiTemplateFactory {
    pub fn new(base_directory: PathBuf, shortcodes_subdirectory: PathBuf) -> Self {
        Self {
            base_directory,
            component_registry: Arc::new(ComponentRegistry::default()),
            shortcodes_subdirectory,
        }
    }

    pub fn register_component_file(&self, file_entry: FileEntry) {
        let component_name = file_entry.get_stem_relative_to(&self.shortcodes_subdirectory);

        self.component_registry
            .register_component(ComponentReference {
                global_fn_name: format!("{}_{}", component_name, rhai_safe_random_affix()),
                name: component_name.clone(),
                path: component_name,
            });
    }
}

impl TryInto<RhaiTemplateRenderer> for RhaiTemplateFactory {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RhaiTemplateRenderer, Self::Error> {
        let evaluator_factory = EvaluatorFactory {
            component_registry: self.component_registry.clone(),
        };

        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
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
        engine.register_fn("clsx", clsx);
        engine.register_fn("error", error);
        engine.register_fn("has", has);
        engine.register_fn("render_hierarchy", render_hierarchy);
        engine.set_max_call_levels(128);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        let templates: DashMap<String, ComponentReference> = DashMap::new();

        for entry in &self.component_registry.components {
            let component_reference = entry.value();

            let module_resolver = engine.module_resolver();
            let module = module_resolver.resolve(
                &engine,
                None,
                &component_reference.path,
                Position::NONE,
            )?;

            engine.register_static_module(component_reference.name.clone(), module);

            templates.insert(
                component_reference.name.clone(),
                component_reference.clone(),
            );
        }

        let meta_module = ComponentMetaModule::from(self.component_registry.clone());

        engine.register_global_module(meta_module.into_global_module(&engine)?.into());

        Ok(RhaiTemplateRenderer::new(
            Arc::new(engine),
            Arc::new(templates),
        ))
    }
}
