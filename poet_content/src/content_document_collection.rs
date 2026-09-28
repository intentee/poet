use std::collections::HashMap;
use std::collections::LinkedList;

use anyhow::Result;
use anyhow::anyhow;
use petgraph::algo::toposort;
use petgraph::stable_graph::StableDiGraph;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_in_collection::ContentDocumentInCollection;
use crate::content_document_tree_node::ContentDocumentTreeNode;

fn find_children(
    collection_name: String,
    parent: &ContentDocumentInCollection,
    sorted_documents: &mut LinkedList<ContentDocumentInCollection>,
) -> LinkedList<ContentDocumentTreeNode> {
    let children = sorted_documents
        .extract_if(|document| {
            document.collection_placement.parent == Some(parent.reference.basename())
        })
        .collect::<LinkedList<_>>();

    children
        .iter()
        .map(|document| ContentDocumentTreeNode {
            children: find_children(collection_name.clone(), document, sorted_documents),
            collection_name: collection_name.clone(),
            reference: document.reference.clone(),
        })
        .collect::<LinkedList<ContentDocumentTreeNode>>()
}

#[derive(Clone, Debug)]
pub struct ContentDocumentCollection {
    pub documents: Vec<ContentDocumentInCollection>,
    pub name: String,
}

impl ContentDocumentCollection {
    /// Returns a list of roots
    pub fn build_hierarchy(&self) -> Result<Vec<ContentDocumentTreeNode>> {
        let mut sorted_documents = self.sort_by_successors()?;
        let roots = sorted_documents
            .extract_if(|document| document.collection_placement.parent.is_none())
            .collect::<LinkedList<ContentDocumentInCollection>>();

        Ok(roots
            .iter()
            .map(|document| ContentDocumentTreeNode {
                children: find_children(self.name.clone(), document, &mut sorted_documents),
                collection_name: self.name.clone(),
                reference: document.reference.clone(),
            })
            .collect::<Vec<_>>())
    }

    pub fn sort_by_successors(&self) -> Result<LinkedList<ContentDocumentInCollection>> {
        let mut basename_to_node = HashMap::new();
        let mut successors_graph: StableDiGraph<ContentDocumentBasename, ()> = StableDiGraph::new();
        let mut node_to_document = HashMap::new();

        // First pass, register all the documents
        for document in &self.documents {
            let node = successors_graph.add_node(document.reference.basename());

            basename_to_node.insert(document.reference.basename(), node);
            node_to_document.insert(node, document.clone());
        }

        // Second pass, register edges
        for document in &self.documents {
            if let Some(after) = &document.collection_placement.after {
                successors_graph.try_add_edge(
                    *basename_to_node.get(after).ok_or(anyhow!(
                        "Unable to find node '{after}' in collection '{}'",
                        self.name
                    ))?,
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
                let mut sorted_documents: LinkedList<ContentDocumentInCollection> =
                    LinkedList::new();

                for node_id in sorted_nodes {
                    sorted_documents.push_back(
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
    use crate::content_document_front_matter::ContentDocumentFrontMatter;
    use crate::content_document_front_matter::collection_placement::CollectionPlacement;
    use crate::content_document_reference::ContentDocumentReference;

    fn create_document_reference(name: &str) -> ContentDocumentReference {
        ContentDocumentReference {
            basename_path: name.into(),
            front_matter: ContentDocumentFrontMatter::mock(name),
            generated_page_base_path: "/".to_string(),
        }
    }

    #[test]
    fn test_sort_by_successors() -> Result<()> {
        let mut collection = ContentDocumentCollection {
            documents: Default::default(),
            name: "my_collection".to_string(),
        };

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: None,
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("1"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string().into()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("5"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string().into()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("4"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("1".to_string().into()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("2"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("2".to_string().into()),
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

    #[test]
    fn test_hierarchy() -> Result<()> {
        let mut collection = ContentDocumentCollection {
            documents: Default::default(),
            name: "my_collection".to_string(),
        };

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: None,
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("1"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string().into()),
                name: "my_collection".to_string(),
                parent: Some("3".to_string().into()),
            },
            reference: create_document_reference("5"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string().into()),
                name: "my_collection".to_string(),
                parent: Some("3".to_string().into()),
            },
            reference: create_document_reference("4"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("1".to_string().into()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("2"),
        });

        collection.documents.push(ContentDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("2".to_string().into()),
                name: "my_collection".to_string(),
                parent: Some("1".to_string().into()),
            },
            reference: create_document_reference("3"),
        });

        let hierarchy = collection.build_hierarchy()?;

        // println!("hierarchy: {hierarchy:#?}");
        // assert!(false);

        let sorted: Vec<String> = hierarchy
            .iter()
            .map(|node| node.reference.front_matter.title.clone())
            .collect();

        assert_eq!(sorted, ["1", "2"]);

        Ok(())
    }
}
