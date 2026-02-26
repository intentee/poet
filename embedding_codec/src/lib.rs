use std::collections::BTreeMap;
use std::str::from_utf8;

use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;

const EMBEDDING_CODEC_VERSION_STRING: &str =
    concat!("embedding_codec_version ", env!("CARGO_PKG_VERSION"));

pub struct EmbeddingCodec;

impl EmbeddingCodec {
    pub fn serialize(embeddings: &BTreeMap<String, Vec<f32>>) -> Vec<u8> {
        let mut buffer = Vec::new();

        buffer.extend_from_slice(&(EMBEDDING_CODEC_VERSION_STRING.len() as u32).to_le_bytes());
        buffer.extend_from_slice(EMBEDDING_CODEC_VERSION_STRING.as_bytes());
        buffer.extend_from_slice(&(embeddings.len() as u32).to_le_bytes());

        for (key, embedding) in embeddings {
            let key_bytes = key.as_bytes();

            buffer.extend_from_slice(&(key_bytes.len() as u32).to_le_bytes());
            buffer.extend_from_slice(key_bytes);
            buffer.extend_from_slice(&(embedding.len() as u32).to_le_bytes());

            for &value in embedding {
                buffer.extend_from_slice(&value.to_le_bytes());
            }
        }

        buffer
    }

    pub fn deserialize(bytes: &[u8]) -> Result<BTreeMap<String, Vec<f32>>> {
        let mut offset = 0;

        let version_length = read_u32_le(bytes, &mut offset)? as usize;

        ensure!(
            offset + version_length <= bytes.len(),
            "unexpected end of data while reading version"
        );

        let file_version = from_utf8(&bytes[offset..offset + version_length])?;

        if file_version != EMBEDDING_CODEC_VERSION_STRING {
            bail!(
                "embedding codec version mismatch: file was written with {file_version}, current version is {EMBEDDING_CODEC_VERSION_STRING}"
            );
        }

        offset += version_length;

        let entry_count = read_u32_le(bytes, &mut offset)? as usize;
        let mut embeddings = BTreeMap::new();

        for _ in 0..entry_count {
            let key_length = read_u32_le(bytes, &mut offset)? as usize;

            ensure!(
                offset + key_length <= bytes.len(),
                "unexpected end of data while reading key"
            );

            let key = from_utf8(&bytes[offset..offset + key_length])?.to_string();
            offset += key_length;

            let embedding_length = read_u32_le(bytes, &mut offset)? as usize;
            let float_byte_length = embedding_length * 4;

            ensure!(
                offset + float_byte_length <= bytes.len(),
                "unexpected end of data while reading embedding"
            );

            let mut embedding = Vec::with_capacity(embedding_length);

            for index in 0..embedding_length {
                let start = offset + index * 4;
                let value = f32::from_le_bytes([
                    bytes[start],
                    bytes[start + 1],
                    bytes[start + 2],
                    bytes[start + 3],
                ]);

                embedding.push(value);
            }

            offset += float_byte_length;
            embeddings.insert(key, embedding);
        }

        if offset != bytes.len() {
            bail!(
                "trailing data: {} bytes remaining after {} entries",
                bytes.len() - offset,
                entry_count
            );
        }

        Ok(embeddings)
    }
}

fn read_u32_le(bytes: &[u8], offset: &mut usize) -> Result<u32> {
    ensure!(
        *offset + 4 <= bytes.len(),
        "unexpected end of data while reading u32 at offset {}",
        offset
    );

    let value = u32::from_le_bytes([
        bytes[*offset],
        bytes[*offset + 1],
        bytes[*offset + 2],
        bytes[*offset + 3],
    ]);
    *offset += 4;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        let mut embeddings = BTreeMap::new();
        embeddings.insert("doc_a".to_string(), vec![1.0, 2.0, 3.0]);
        embeddings.insert("doc_b".to_string(), vec![0.5, -0.5]);
        embeddings.insert("empty".to_string(), vec![]);

        let bytes = EmbeddingCodec::serialize(&embeddings);
        let result = EmbeddingCodec::deserialize(&bytes).unwrap();

        assert_eq!(embeddings, result);
    }

    #[test]
    fn test_empty_map() {
        let embeddings = BTreeMap::new();
        let bytes = EmbeddingCodec::serialize(&embeddings);
        let result = EmbeddingCodec::deserialize(&bytes).unwrap();

        assert_eq!(embeddings, result);
    }

    #[test]
    fn test_truncated_data_is_rejected() {
        let mut embeddings = BTreeMap::new();
        embeddings.insert("key".to_string(), vec![1.0]);

        let bytes = EmbeddingCodec::serialize(&embeddings);
        let truncated = &bytes[..bytes.len() - 2];

        assert!(EmbeddingCodec::deserialize(truncated).is_err());
    }

    #[test]
    fn test_trailing_data_is_rejected() {
        let mut embeddings = BTreeMap::new();
        embeddings.insert("key".to_string(), vec![1.0]);

        let mut bytes = EmbeddingCodec::serialize(&embeddings);
        bytes.push(0xFF);

        assert!(EmbeddingCodec::deserialize(&bytes).is_err());
    }

    #[test]
    fn test_version_mismatch_is_rejected() {
        let mut bytes = Vec::new();
        let fake_version = b"99.99.99";

        bytes.extend_from_slice(&(fake_version.len() as u32).to_le_bytes());
        bytes.extend_from_slice(fake_version);
        bytes.extend_from_slice(&0u32.to_le_bytes());

        let error = EmbeddingCodec::deserialize(&bytes).unwrap_err();

        assert!(error.to_string().contains("version mismatch"));
    }

    #[test]
    fn test_serialized_data_starts_with_version() {
        let embeddings = BTreeMap::new();
        let bytes = EmbeddingCodec::serialize(&embeddings);
        let version_length = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        let version = from_utf8(&bytes[4..4 + version_length]).unwrap();

        assert_eq!(version, EMBEDDING_CODEC_VERSION_STRING);
    }
}
