use poet_assets::is_external_link::is_external_link;

#[test]
fn treats_relative_path_as_internal() {
    assert!(!is_external_link("assets/main.js"));
}
