use std::path::PathBuf;

use anyhow::Result;
use rhai::Dynamic;
use rhai::Engine;
use rhai::Func;
use rhai::OptimizationLevel;
use rhai::module_resolvers::FileModuleResolver;

use crate::asset_manager::AssetManager;
use crate::filesystem::file_entry::FileEntry;

pub type ShortcodeRenderer = dyn Fn(AssetManager, Dynamic, Dynamic) -> Result<String>;

pub struct RhaiContext {
    scripts_directory: PathBuf,
}

impl RhaiContext {
    pub fn new(scripts_directory: PathBuf) -> Self {
        Self { scripts_directory }
    }

    pub fn compile_shortcode_file(&self, file: &FileEntry) -> Result<Box<ShortcodeRenderer>> {
        let renderer = Func::<(AssetManager, Dynamic, Dynamic), String>::create_from_script(
            // closure consumes the engine
            self.create_engine(),
            &file.contents,
            "template",
        )?;

        Ok(Box::new(
            move |assets: AssetManager, content: Dynamic, props: Dynamic| -> Result<String> {
                Ok(renderer(assets, content, props)?)
            },
        ))
    }

    fn create_engine(&self) -> Engine {
        let mut engine = Engine::new();

        engine.set_fail_on_invalid_map_property(true);
        engine.set_module_resolver(FileModuleResolver::new_with_path(&self.scripts_directory));
        engine.set_optimization_level(OptimizationLevel::Full);
        engine.set_strict_variables(true);

        engine
    }
}
