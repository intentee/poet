use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet::poet_error::PoetError;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::resource_provider_list_params::ResourceProviderListParams;

#[tokio::test]
async fn rejects_resources_until_project_is_built() {
    let provider = McpResourceProviderContentDocuments::default();

    assert_eq!(provider.total(), 0);

    let Err(provider_error) = provider
        .list_resources(ResourceProviderListParams {
            limit: 1,
            offset: 0,
        })
        .await
    else {
        panic!("expected resources to be unavailable");
    };

    assert!(matches!(
        provider_error.downcast_ref::<PoetError>(),
        Some(PoetError::BuildProjectResultNotReady)
    ));
}
