use poet_mcp::role::Role;

#[test]
fn rejects_unknown_role_name() {
    assert!("system".parse::<Role>().is_err());
}
