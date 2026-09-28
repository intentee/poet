use poet_mcp::log_level::LogLevel;

#[test]
fn orders_log_levels_from_least_to_most_severe() {
    let mut log_levels = vec![
        LogLevel::Emergency,
        LogLevel::Debug,
        LogLevel::Warning,
        LogLevel::Info,
        LogLevel::Alert,
        LogLevel::Notice,
        LogLevel::Critical,
        LogLevel::Error,
    ];

    log_levels.sort();

    assert_eq!(
        log_levels,
        vec![
            LogLevel::Debug,
            LogLevel::Info,
            LogLevel::Notice,
            LogLevel::Warning,
            LogLevel::Error,
            LogLevel::Critical,
            LogLevel::Alert,
            LogLevel::Emergency,
        ]
    );
}
