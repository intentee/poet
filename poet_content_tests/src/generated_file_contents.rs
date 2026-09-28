use std::path::Path;

use poet_content::build_project_result_stub::BuildProjectResultStub;
use poet_content::generated_file::GeneratedFile;

use crate::poet_content_tests_error::PoetContentTestsError;

pub fn generated_file_contents(
    build_project_result_stub: &BuildProjectResultStub,
    relative_path: &str,
) -> Result<String, PoetContentTestsError> {
    build_project_result_stub
        .generated_files
        .iter()
        .find(|generated_file| generated_file.relative_path == Path::new(relative_path))
        .map(|GeneratedFile { contents, .. }| contents.clone())
        .ok_or_else(|| PoetContentTestsError::MissingGeneratedFile {
            relative_path: relative_path.to_owned(),
        })
}
