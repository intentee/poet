use std::sync::Arc;

use super::output_filesystem_holder::OutputFilesystemHolder;
use crate::filesystem::memory::Memory;

pub struct AppData {
    pub output_filesystem_holder: Arc<OutputFilesystemHolder<Memory>>,
}
