use std::path::PathBuf;

use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;
use poet_content::generated_file::GeneratedFile;
use poet_content::generated_file_kind::GeneratedFileKind;

fn generated_file(kind: GeneratedFileKind, relative_path: &str) -> GeneratedFile {
    GeneratedFile {
        contents: relative_path.to_owned(),
        kind,
        relative_path: PathBuf::from(relative_path),
    }
}

fn routed_path(
    filesystem_http_route_index: &FilesystemHttpRouteIndex,
    route: &str,
) -> Option<PathBuf> {
    filesystem_http_route_index
        .generated_file_for_route(route)
        .map(|routed_file| routed_file.relative_path.clone())
}

#[test]
fn routes_generated_pages_by_directory_and_file_name() {
    let filesystem_http_route_index = FilesystemHttpRouteIndex::from_generated_files(&[
        generated_file(GeneratedFileKind::Page, "index.html"),
        generated_file(GeneratedFileKind::Page, "docs/index.html"),
        generated_file(GeneratedFileKind::Sitemap, "sitemap.xml"),
    ]);

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
}
