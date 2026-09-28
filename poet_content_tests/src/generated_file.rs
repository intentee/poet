use std::path::Path;

use poet_content::build_project_result_stub::BuildProjectResultStub;
use poet_filesystem::filesystem::Filesystem as _;

use crate::poet_content_tests_error::PoetContentTestsError;

pub async fn generated_file(
    build_project_result_stub: &BuildProjectResultStub,
    relative_path: &str,
) -> Result<String, PoetContentTestsError> {
    Ok(build_project_result_stub
        .memory_filesystem
        .read_file_contents_string(Path::new(relative_path))
        .await?)
}
