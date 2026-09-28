use poet_error_chain::error_chain::ErrorChain;
use poet_error_chain_tests::layered_error::LayeredError;

#[test]
fn renders_every_cause_of_error_chain() {
    let layered_error = LayeredError::Publish(Box::new(LayeredError::Render {
        page: "guide".to_owned(),
        source: Box::new(LayeredError::TemplateNotFound {
            template: "Layout".to_owned(),
        }),
    }));

    assert_eq!(
        ErrorChain {
            error: &layered_error
        }
        .to_string(),
        "unable to publish the site: unable to render page 'guide': template 'Layout' does not exist"
    );
}
