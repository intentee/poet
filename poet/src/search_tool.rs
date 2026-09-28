use std::sync::Arc;

use async_trait::async_trait;
use poet_content::content_document_reference::ContentDocumentReference;
use poet_mcp::content_block::ContentBlock;
use poet_mcp::provider_error::ProviderError;
use poet_mcp::resource_link::ResourceLink;
use poet_mcp::resource_provider::ResourceProvider as _;
use poet_mcp::tool_call_error_message::ToolCallErrorMessage;
use poet_mcp::tool_call_result::ToolCallResult;
use poet_mcp::tool_call_success::ToolCallSuccess;
use poet_mcp::tool_provider::ToolProvider;
use poet_mcp::tool_responder::ToolResponder;
use poet_search::search_index_found_document::SearchIndexFoundDocument;
use poet_search::search_index_query_params::SearchIndexQueryParams;
use poet_search::search_index_reader::SearchIndexReader;
use tokio::task::spawn_blocking;

use crate::holder::Holder;
use crate::holder_state::HolderState;
use crate::mcp_resource_provider_content_documents::McpResourceProviderContentDocuments;
use crate::search_tool_provider_input::SearchToolProviderInput;
use crate::search_tool_provider_output::SearchToolProviderOutput;

const SEARCH_RESULTS_PER_PAGE: usize = 20;

pub struct SearchTool {
    pub mcp_resource_provider_content_documents: McpResourceProviderContentDocuments,
    pub search_index_reader_holder: Holder<Arc<SearchIndexReader>>,
}

impl SearchTool {
    fn resource_link(
        &self,
        SearchIndexFoundDocument {
            content_document_reference:
                content_document_reference @ ContentDocumentReference { front_matter, .. },
        }: &SearchIndexFoundDocument,
    ) -> ContentBlock {
        ContentBlock::ResourceLink(ResourceLink {
            description: Some(front_matter.description.clone()),
            mime_type: Some("text/markdown".to_owned()),
            name: front_matter.title.clone(),
            title: Some(front_matter.title.clone()),
            uri: self
                .mcp_resource_provider_content_documents
                .resource_uri(&content_document_reference.basename().to_string()),
        })
    }
}

impl ToolProvider for SearchTool {
    type Input = SearchToolProviderInput;
    type Output = SearchToolProviderOutput;

    fn name(&self) -> String {
        "search".to_owned()
    }
}

#[async_trait]
impl ToolResponder<Self> for SearchTool {
    async fn respond(
        &self,
        SearchToolProviderInput { query }: SearchToolProviderInput,
    ) -> Result<ToolCallResult<SearchToolProviderOutput>, ProviderError> {
        let HolderState::Ready(search_index_reader) = self.search_index_reader_holder.get() else {
            return Ok(ToolCallErrorMessage(
                "Search index is not ready yet. There are no successful builds yet, or the server needs more time to start.",
            )
            .into());
        };
        let search_index_found_documents = spawn_blocking(move || {
            search_index_reader.query(SearchIndexQueryParams {
                offset: 0,
                per_page: SEARCH_RESULTS_PER_PAGE,
                query,
            })
        })
        .await??;

        Ok(ToolCallResult::Success(ToolCallSuccess {
            content: search_index_found_documents
                .iter()
                .map(|search_index_found_document| self.resource_link(search_index_found_document))
                .collect(),
            structured_content: SearchToolProviderOutput {},
        }))
    }
}
