use std::path::Path;
use std::path::PathBuf;

use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;
use poet_filesystem::memory::Memory;

use crate::poet_tests_error::PoetTestsError;

fn routed_path(
    filesystem_http_route_index: &FilesystemHttpRouteIndex,
    route: &str,
) -> Option<PathBuf> {
    filesystem_http_route_index
        .file_entry_for_route(route)
        .map(|file_entry| file_entry.relative_path.clone())
}

#[test]
fn routes_generated_pages_by_directory_and_file_name() -> Result<(), PoetTestsError> {
    let memory_filesystem = Memory::default();

    memory_filesystem.set_file_contents_sync(Path::new("index.html"), "home");
    memory_filesystem.set_file_contents_sync(Path::new("docs/index.html"), "docs");
    memory_filesystem.set_file_contents_sync(Path::new("sitemap.xml"), "sitemap");

    let filesystem_http_route_index = FilesystemHttpRouteIndex::from_memory(&memory_filesystem)?;

    assert_eq!(
        routed_path(&filesystem_http_route_index, ""),
        Some(PathBuf::from("index.html"))
    );
    assert_eq!(
        routed_path(&filesystem_http_route_index, "index.html"),
        Some(PathBuf::from("index.html"))
    );
    assert_eq!(
        routed_path(&filesystem_http_route_index, "docs/"),
        Some(PathBuf::from("docs/index.html"))
    );
    assert_eq!(
        routed_path(&filesystem_http_route_index, "docs/index.html"),
        Some(PathBuf::from("docs/index.html"))
    );
    assert_eq!(
        routed_path(&filesystem_http_route_index, "sitemap.xml"),
        Some(PathBuf::from("sitemap.xml"))
    );
    assert_eq!(routed_path(&filesystem_http_route_index, "missing"), None);

    Ok(())
}
