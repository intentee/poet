use poet_mcp::tool_call_failure::ToolCallFailure;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp_tests::poet_mcp_tests_error::PoetMcpTestsError;

#[test]
fn keeps_tool_call_failure_when_converting_into_json_value() -> Result<(), PoetMcpTestsError> {
    let tool_call_result: ToolCallResult<u32> =
        ToolCallResult::Failure(ToolCallFailure { content: vec![] });

    assert!(matches!(
        tool_call_result.try_into_value()?,
        ToolCallResult::Failure(ToolCallFailure { content }) if content.is_empty()
    ));

    Ok(())
}
