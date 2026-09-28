use std::pin::pin;
use std::sync::Arc;

use poet::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use poet_content::build_project_result::BuildProjectResult;
use poet_content_tests::build_fixture_guide::build_fixture_guide;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::resource_reference::ResourceReference;
use tokio::task::yield_now;
use tokio_util::sync::CancellationToken;

#[actix_web::test]
async fn notifies_subscribers_about_changed_document() -> Result<(), ProviderError> {
    let provider = Arc::new(McpResourceProviderContentDocuments::default());
    let previous_build: BuildProjectResult = build_fixture_guide("Guide description", "body")
        .await?
        .into();

    provider
        .build_project_result_holder
        .set(previous_build.clone());

    let resource_update_notifier = provider.clone().resource_update_notifier(
        CancellationToken::new(),
        ResourceReference {
            class: "content".to_owned(),
            path: "guide".to_owned(),
            scheme: "poet".to_owned(),
            uri_string: "poet://content/guide".to_owned(),
        },
    );
    let mut resource_updated = pin!(resource_update_notifier.notified());

    resource_updated.as_mut().enable();
    yield_now().await;

    provider.build_project_result_holder.set(
        build_fixture_guide("Guide description", "changed body")
            .await?
            .changed_compared_to(&previous_build),
    );

    resource_updated.await;

    Ok(())
}
