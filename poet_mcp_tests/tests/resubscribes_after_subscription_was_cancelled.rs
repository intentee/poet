use poet_mcp::session_subscriptions::SessionSubscriptions;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[test]
fn resubscribes_after_subscription_was_cancelled() -> Result<(), PoetMcpTestsError> {
    let session_subscriptions = SessionSubscriptions::default();

    session_subscriptions
        .subscribe("memory://documents/first")?
        .cancel();

    assert!(
        !session_subscriptions
            .subscribe("memory://documents/first")?
            .is_cancelled()
    );

    Ok(())
}
