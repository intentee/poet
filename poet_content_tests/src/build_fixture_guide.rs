use poet_content::build_project_result_stub::BuildProjectResultStub;

use crate::fixture_project::FixtureProject;
use crate::layout_plain_shortcode::LAYOUT_PLAIN_SHORTCODE;
use crate::poet_content_tests_error::PoetContentTestsError;

pub async fn build_fixture_guide(
    description: &str,
    body: &str,
) -> Result<BuildProjectResultStub, PoetContentTestsError> {
    let fixture_project = FixtureProject::create()?;

    fixture_project
        .add_file("shortcodes/LayoutPlain.rhai", LAYOUT_PLAIN_SHORTCODE)
        .await?;
    fixture_project
        .add_file(
            "content/guide.md",
            &format!(
                "+++\ndescription = \"{description}\"\nlayout = \"LayoutPlain\"\ntitle = \"Guide\"\n+++\n\n{body}\n"
            ),
        )
        .await?;

    fixture_project.build(false).await
}
