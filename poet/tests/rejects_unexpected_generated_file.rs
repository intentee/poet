use std::path::Path;

use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;
use poet::poet_error::PoetError;
use poet_filesystem::memory::Memory;

#[test]
fn rejects_unexpected_generated_file() {
    let memory_filesystem = Memory::default();

    memory_filesystem.set_file_contents_sync(Path::new("page.html"), "page");

    assert!(matches!(
        FilesystemHttpRouteIndex::from_memory(&memory_filesystem),
        Err(PoetError::UnexpectedGeneratedFile { relative_path }) if relative_path == Path::new("page.html")
    ));
}
