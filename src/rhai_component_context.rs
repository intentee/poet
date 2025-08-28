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
}

impl CustomType for RhaiComponentContext {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiComponentContext")
            .with_get("assets", Self::get_assets)
            .with_get("file", Self::get_file)
            .with_get("front_matter", Self::get_front_matter);
    }
}
