use std::collections::HashMap;

use anyhow::Result;
use anyhow::anyhow;
use petgraph::algo::toposort;
use petgraph::stable_graph::StableDiGraph;

use crate::markdown_document_in_collection::MarkdownDocumentInCollection;

#[derive(Clone, Debug, Default)]
pub struct MarkdownDocumentCollection {
    pub documents: Vec<MarkdownDocumentInCollection>,
}

impl MarkdownDocumentCollection {
    pub fn sort_by_successors(&self) -> Result<Vec<MarkdownDocumentInCollection>> {
        let mut basename_to_node = HashMap::new();
        let mut successors_graph: StableDiGraph<String, ()> = StableDiGraph::new();
        let mut node_to_document = HashMap::new();

        // First pass, register all the documents
        for document in &self.documents {
            let node = successors_graph.add_node(document.reference.basename());

            basename_to_node.insert(document.reference.basename(), node);
            node_to_document.insert(node, document.clone());
        }

        // Second pass, register edges
        for document in &self.documents {
            if let Some(after) = &document.collection.after {
                successors_graph.try_add_edge(
                    *basename_to_node
                        .get(after)
                        .ok_or(anyhow!("Unable to find node {}", after))?,
                    *basename_to_node
                        .get(&document.reference.basename())
                        .ok_or(anyhow!(
                            "Unable to find node {}",
                            document.reference.basename()
                        ))?,
                    (),
                )?;
            }
        }

        match toposort(&successors_graph, None) {
            Ok(sorted_nodes) => {
                let mut sorted_documents: Vec<MarkdownDocumentInCollection> = Vec::new();

                for node_id in sorted_nodes {
                    sorted_documents.push(
                        node_to_document
                            .get(&node_id)
                            .ok_or(anyhow!("Unable to inverse find document for node"))?
                            .clone(),
                    );
                }

                Ok(sorted_documents)
            }
            Err(_) => Err(anyhow!("Found cycle in documents successors")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_matter::FrontMatter;
    use crate::front_matter::collection::Collection;
    use crate::markdown_document_reference::MarkdownDocumentReference;

    fn create_document_reference(name: &str) -> MarkdownDocumentReference {
        MarkdownDocumentReference {
            basename_path: name.into(),
            front_matter: FrontMatter::mock(name),
        }
    }

    #[test]
    fn test_sort_by_successors() -> Result<()> {
        let mut collection = MarkdownDocumentCollection::default();

        collection.documents.push(MarkdownDocumentInCollection {
            collection: Collection {
                after: None,
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("1"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection: Collection {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("5"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection: Collection {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("4"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection: Collection {
                after: Some("1".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("2"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection: Collection {
                after: Some("2".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("3"),
        });

        let sorted: Vec<String> = collection
            .sort_by_successors()?
            .iter()
            .map(|document| document.reference.front_matter.title.clone())
            .collect();

        assert_eq!(sorted, ["1", "2", "3", "4", "5"]);

        Ok(())
    }
}
