use rhai::LexError;
use rhai::ParseErrorType;
use rhai_components_tests::fixtures_engine::fixtures_engine;

#[test]
fn rejects_mismatched_closing_tag() {
    assert!(
        fixtures_engine()
            .compile("component { <div></span> }")
            .is_err_and(|parse_error| matches!(
                parse_error.err_type(),
                ParseErrorType::BadInput(LexError::UnexpectedInput(message))
                    if message == "Mismatched closing tag: expected </div>, got </span>"
            ))
    );
}
