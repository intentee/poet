use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet_content_tests::build_fixture_guide::build_fixture_guide;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource_content::ResourceContent;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::resource_reference::ResourceReference;
use poet_mcp::text_resource_content::TextResourceContent;

fn resource_reference(path: &str) -> ResourceReference {
    ResourceReference {
        class: "content".to_owned(),
        path: path.to_owned(),
        scheme: "poet".to_owned(),
        uri_string: format!("poet://content/{path}"),
    }
}

#[tokio::test]
async fn reads_content_document_resource() -> Result<(), ProviderError> {
    let provider = McpResourceProviderContentDocuments::default();

    provider.build_project_result_holder.set(
        build_fixture_guide("Guide description", "body")
            .await?
            .into(),
    );

    assert!(matches!(
        provider.read_resource_contents(resource_reference("guide")).await?.as_deref(),
        Some([ResourceContent::Text(TextResourceContent { text, .. })]) if text.contains("body")
    ));
    assert!(
        provider
            .read_resource_contents(resource_reference("missing"))
            .await?
            .is_none()
    );

    Ok(())
}
