use poet::holder::Holder;
use poet::holder_state::HolderState;

#[test]
fn holder_tracks_readiness_of_held_item() {
    let holder: Holder<String> = Holder::default();

    assert!(matches!(holder.get(), HolderState::NotReady));

    holder.set("ready".to_owned());

    assert!(matches!(holder.get(), HolderState::Ready(item) if item == "ready"));

    holder.reset();

    assert!(matches!(holder.get(), HolderState::NotReady));
}
