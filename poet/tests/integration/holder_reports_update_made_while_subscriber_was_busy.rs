use futures_util::FutureExt as _;
use poet::holder::Holder;

#[test]
fn holder_reports_update_made_while_subscriber_was_busy() {
    let holder: Holder<String> = Holder::default();
    let mut holder_updates = holder.subscribe();

    holder.set("rebuilt".to_owned());

    assert!(matches!(
        holder_updates.changed().now_or_never(),
        Some(Ok(()))
    ));
}
