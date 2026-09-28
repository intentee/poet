use syntect::parsing::SyntaxDefinition;
use syntect::parsing::SyntaxSet;
use syntect::parsing::SyntaxSetBuilder;

use crate::poet_content_tests_error::PoetContentTestsError;

pub fn fixture_syntax_set(syntax_definition: &str) -> Result<SyntaxSet, PoetContentTestsError> {
    let mut syntax_set_builder = SyntaxSetBuilder::new();

    syntax_set_builder.add(SyntaxDefinition::load_from_str(
        syntax_definition,
        true,
        None,
    )?);

    Ok(syntax_set_builder.build())
}
