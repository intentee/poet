use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context as _;
use anyhow::Result;
use dashmap::DashMap;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Func;
use rhai::module_resolvers::FileModuleResolver;

use crate::asset_manager::AssetManager;
use crate::component_context::ComponentContext;
use crate::filesystem::file_entry::FileEntry;
use crate::front_matter::collection_placement::CollectionPlacement;
use crate::rhai_components::component_meta_module::ComponentMetaModule;
use crate::rhai_components::component_reference::ComponentReference;
use crate::rhai_components::component_registry::ComponentRegistry;
use crate::rhai_components::evaluator_factory::EvaluatorFactory;
use crate::rhai_components::parse_component::parse_component;
use crate::rhai_front_matter::RhaiFrontMatter;
use crate::rhai_front_matter::rhai_collection_placement_list::RhaiCollectionPlacementList;
use crate::rhai_functions::clsx;
use crate::rhai_functions::error;
use crate::rhai_functions::render_hierarchy;
use crate::rhai_markdown_document_collection::RhaiMarkdownDocumentCollection;
use crate::rhai_markdown_document_hierarchy::RhaiMarkdownDocumentHierarchy;
use crate::rhai_markdown_document_reference::RhaiMarkdownDocumentReference;
use crate::rhai_markdown_document_tree_node::RhaiMarkdownDocumentTreeNode;
use crate::rhai_safe_random_affix::rhai_safe_random_affix;
use crate::rhai_template_renderer::RhaiTemplateRenderer;
use crate::rhai_template_renderer::ShortcodeRenderer;
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
                file_entry,
                global_fn_name: format!("{}_{}", component_name, rhai_safe_random_affix()),
                name: component_name.clone(),
                path: component_name,
            });
    }

    fn prepare_engine(&self) -> Result<Engine> {
        let evaluator_factory = EvaluatorFactory {
            component_registry: self.component_registry.clone(),
        };

        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_max_expr_depths(256, 256);
        engine.set_module_resolver(FileModuleResolver::new_with_path(
            self.base_directory.join(&self.shortcodes_subdirectory),
        ));

        engine.build_type::<AssetManager>();
        engine.build_type::<CollectionPlacement>();
        engine.build_type::<ComponentContext>();
        engine.build_type::<FileEntry>();
        engine.build_type::<Heading>();
        engine.build_type::<RhaiCollectionPlacementList>();
        engine.build_type::<RhaiFrontMatter>();
        engine.build_type::<RhaiMarkdownDocumentCollection>();
        engine.build_type::<RhaiMarkdownDocumentHierarchy>();
        engine.build_type::<RhaiMarkdownDocumentReference>();
        engine.build_type::<RhaiMarkdownDocumentTreeNode>();
        engine.build_type::<TableOfContents>();
        engine.register_fn("clsx", clsx);
        engine.register_fn("error", error);
        engine.register_fn("render_hierarchy", render_hierarchy);

        engine.register_custom_syntax_without_look_ahead_raw(
            "component",
            parse_component,
            true,
            evaluator_factory.create_component_evaluator(),
        );

        Ok(engine)
    }
}

impl TryInto<RhaiTemplateRenderer> for RhaiTemplateFactory {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<RhaiTemplateRenderer, Self::Error> {
        let templates: DashMap<String, Box<ShortcodeRenderer>> = DashMap::new();

        for entry in &self.component_registry.components {
            let component_reference = entry.value();
            let meta_module = ComponentMetaModule::from(self.component_registry.clone());
            let mut engine = self.prepare_engine()?;

            engine.register_global_module(meta_module.into_global_module(&engine)?.into());

            let renderer =
                Func::<(ComponentContext, Dynamic, Dynamic), String>::create_from_script(
                    // closure consumes the engine
                    engine,
                    &component_reference.file_entry.contents,
                    "template",
                )?;

            let template_relative_path = component_reference
                .file_entry
                .relative_path
                .display()
                .to_string();

            templates.insert(
                component_reference.name.clone(),
                Box::new(
                    move |context: ComponentContext,
                          content: Dynamic,
                          props: Dynamic|
                          -> Result<String> {
                        renderer(context, content, props).context(format!(
                            "Shortcode rendering failed: {template_relative_path}",
                        ))
                    },
                ),
            );
        }

        Ok(RhaiTemplateRenderer::new(self.prepare_engine()?, templates))
    }
}
