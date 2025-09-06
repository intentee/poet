use std::sync::Arc;

use esbuild_metafile::EsbuildMetaFile;

use crate::filesystem::memory::Memory;

pub struct BuildProjectResult {
    pub esbuild_metafile: Arc<EsbuildMetaFile>,
    pub memory_filesystem: Memory,
}
