use rhai::CustomType;
use rhai::TypeBuilder;

use crate::asset_manager::AssetManager;
use crate::filesystem::file_entry::FileEntry;
use crate::front_matter::FrontMatter;

#[derive(Clone)]
pub struct RhaiComponentContext {
    pub asset_manager: AssetManager,
    pub file_entry: FileEntry,
    pub front_matter: FrontMatter,
    pub is_watching: bool,
}

impl RhaiComponentContext {
    pub fn get_assets(&mut self) -> AssetManager {
        self.asset_manager.clone()
    }

    pub fn get_file(&mut self) -> FileEntry {
        self.file_entry.clone()
    }

    pub fn get_front_matter(&mut self) -> FrontMatter {
        self.front_matter.clone()
    }

    pub fn get_is_watching(&mut self) -> bool {
        self.is_watching
    }

    pub fn link_to(&mut self, path: &str) -> String {
        path.to_string()
    }
}

impl CustomType for RhaiComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiComponentContext")
            .with_get("assets", Self::get_assets)
            .with_get("file", Self::get_file)
            .with_get("front_matter", Self::get_front_matter)
            .with_get("is_watching", Self::get_is_watching)
            .with_fn("link_to", Self::link_to);
    }
}
