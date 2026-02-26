use anyhow::Result;
use anyhow::anyhow;
use futures_util::StreamExt;
use paddler_client::PaddlerClient;
use paddler_types::embedding::Embedding;
use paddler_types::embedding_input_document::EmbeddingInputDocument;
use paddler_types::embedding_normalization_method::EmbeddingNormalizationMethod;
use paddler_types::embedding_result::EmbeddingResult;
use paddler_types::inference_client::Message as InferenceMessage;
use paddler_types::inference_client::Response as InferenceResponse;
use paddler_types::request_params::GenerateEmbeddingBatchParams;
use url::Url;

pub struct PaddlerEmbeddingClient {
    paddler_client: PaddlerClient,
}

impl PaddlerEmbeddingClient {
    pub fn new(inference_addr: Url) -> Self {
        let paddler_client = PaddlerClient::new(inference_addr.clone(), inference_addr, 1);

        Self { paddler_client }
    }

    pub async fn generate_embeddings(
        &self,
        documents: Vec<EmbeddingInputDocument>,
    ) -> Result<Vec<Embedding>> {
        let params = GenerateEmbeddingBatchParams {
            input_batch: documents,
            normalization_method: EmbeddingNormalizationMethod::L2,
        };

        let mut stream = self
            .paddler_client
            .inference()
            .generate_embedding_batch(&params)
            .await
            .map_err(|err| anyhow!("{err}"))?;

        let mut embeddings: Vec<Embedding> = Vec::new();

        while let Some(message_result) = stream.next().await {
            let message = message_result.map_err(|err| anyhow!("{err}"))?;

            match message {
                InferenceMessage::Response(envelope) => match envelope.response {
                    InferenceResponse::Embedding(EmbeddingResult::Embedding(embedding)) => {
                        embeddings.push(embedding);
                    }
                    InferenceResponse::Embedding(EmbeddingResult::Done) => {
                        break;
                    }
                    InferenceResponse::Embedding(EmbeddingResult::Error(error)) => {
                        return Err(anyhow!("Embedding error: {error}"));
                    }
                    InferenceResponse::GeneratedToken(_) => {}
                    InferenceResponse::Timeout => {
                        return Err(anyhow!("Embedding request timed out"));
                    }
                    InferenceResponse::TooManyBufferedRequests => {
                        return Err(anyhow!("Too many buffered requests"));
                    }
                },
                InferenceMessage::Error(error_envelope) => {
                    return Err(anyhow!(
                        "Paddler error: {}",
                        error_envelope.error.description
                    ));
                }
            }
        }

        Ok(embeddings)
    }
}
