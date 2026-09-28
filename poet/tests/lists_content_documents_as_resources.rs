use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet_content_tests::build_fixture_guide::build_fixture_guide;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource::Resource;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::resource_provider_list_params::ResourceProviderListParams;

#[tokio::test]
async fn lists_content_documents_as_resources() -> Result<(), ProviderError> {
    let provider = McpResourceProviderContentDocuments::default();

    provider.build_project_result_holder.set(
        build_fixture_guide("Guide description", "body")
            .await?
            .into(),
    );

    assert_eq!(provider.total(), 1);
    assert!(matches!(
        provider
            .list_resources(ResourceProviderListParams {
                limit: 1,
                offset: 0,
            })
            .await?
            .as_slice(),
        [Resource { name, title, uri, .. }] if name == "guide" && title == "Guide" && uri == "poet://content/guide"
    ));

    Ok(())
}
