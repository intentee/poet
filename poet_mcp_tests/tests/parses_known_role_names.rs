use poet_mcp::role::Role;

#[test]
fn parses_known_role_names() {
    assert!(matches!("assistant".parse::<Role>(), Ok(Role::Assistant)));
    assert!(matches!("user".parse::<Role>(), Ok(Role::User)));
}
