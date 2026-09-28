use poet_mcp::implementation::Implementation;

#[must_use]
pub fn fixtures_server_info() -> Implementation {
    Implementation {
        description: None,
        name: "fixtures".to_owned(),
        title: Some("Fixtures".to_owned()),
        version: "1.0.0".to_owned(),
    }
}
