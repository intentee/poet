use std::path::PathBuf;
use std::sync::Arc;

use actix_web::body::to_bytes;
use actix_web::http::StatusCode;
use actix_web::http::header::CONTENT_TYPE;
use poet::cmd::respond_with_generated_page_holder::respond_with_generated_page_holder;
use poet::filesystem_http_route_index::FilesystemHttpRouteIndex;
use poet::holder::Holder;
use poet_content::generated_file::GeneratedFile;
use poet_content::generated_file_kind::GeneratedFileKind;

#[actix_web::test]
async fn responds_with_generated_page() {
    let filesystem_http_route_index_holder = Holder::default();

    filesystem_http_route_index_holder.set(Arc::new(
        FilesystemHttpRouteIndex::from_generated_files(&[GeneratedFile {
            contents: "<html>home</html>".to_owned(),
            kind: GeneratedFileKind::Page,
            relative_path: PathBuf::from("index.html"),
        }]),
    ));

    let response = respond_with_generated_page_holder(&filesystem_http_route_index_holder, "");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .map(actix_web::http::header::HeaderValue::as_bytes),
        Some(b"text/html".as_slice())
    );
    assert!(matches!(
        to_bytes(response.into_body()).await,
        Ok(body) if body == "<html>home</html>"
    ));
}
