use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use embedding_codec::EmbeddingCodec;
use log::debug;

pub struct SemanticSearchIndex {
    embeddings: BTreeMap<String, Vec<f32>>,
}

impl SemanticSearchIndex {
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let bytes = fs::read(path)?;
        let embeddings = EmbeddingCodec::deserialize(&bytes)?;

        Ok(Self { embeddings })
    }

    pub fn query(&self, embedding: &[f32], top_k: usize, min_score: f32) -> Vec<(String, f32)> {
        let mut scored: Vec<(String, f32)> = self
            .embeddings
            .iter()
            .map(|(basename, stored_embedding)| {
                let score = cosine_similarity(embedding, stored_embedding);

                (basename.clone(), score)
            })
            .inspect(|(basename, score)| {
                debug!("Semantic search score for {basename}: {score:.6}");
            })
            .filter(|(_basename, score)| *score >= min_score)
            .collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));
        scored.truncate(top_k);

        scored
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let mag_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }

    dot / (mag_a * mag_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cosine_similarity_identical() {
        let a = vec![1.0, 2.0, 3.0];

        assert!((cosine_similarity(&a, &a) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];

        assert!(cosine_similarity(&a, &b).abs() < 1e-6);
    }

    #[test]
    fn test_cosine_similarity_zero_vector() {
        let a = vec![1.0, 2.0];
        let b = vec![0.0, 0.0];

        assert_eq!(cosine_similarity(&a, &b), 0.0);
    }

    #[test]
    fn test_query_returns_top_k() {
        let mut embeddings = BTreeMap::new();
        embeddings.insert("doc_a".to_string(), vec![1.0, 0.0]);
        embeddings.insert("doc_b".to_string(), vec![0.7, 0.7]);
        embeddings.insert("doc_c".to_string(), vec![0.0, 1.0]);

        let index = SemanticSearchIndex { embeddings };
        let results = index.query(&[1.0, 0.0], 2, 0.0);

        assert_eq!(results.len(), 2);
        assert_eq!(results[0].0, "doc_a");
    }

    #[test]
    fn test_query_filters_by_min_score() {
        let mut embeddings = BTreeMap::new();
        embeddings.insert("doc_a".to_string(), vec![1.0, 0.0]);
        embeddings.insert("doc_b".to_string(), vec![0.7, 0.7]);
        embeddings.insert("doc_c".to_string(), vec![0.0, 1.0]);

        let index = SemanticSearchIndex { embeddings };
        let results = index.query(&[1.0, 0.0], 10, 0.8);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].0, "doc_a");
    }
}
