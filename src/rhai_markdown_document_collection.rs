use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::LinkedList;
use std::sync::Arc;

use anyhow::Result;
use anyhow::anyhow;
use petgraph::algo::toposort;
use petgraph::stable_graph::StableDiGraph;
use rhai::CustomType;
use rhai::EvalAltResult;
use rhai::TypeBuilder;

use crate::markdown_document_in_collection::MarkdownDocumentInCollection;
use crate::rhai_front_matter::RhaiFrontMatter;
use crate::rhai_markdown_document_hierarchy::RhaiMarkdownDocumentHierarchy;
use crate::rhai_markdown_document_reference::RhaiMarkdownDocumentReference;
use crate::rhai_markdown_document_tree_node::RhaiMarkdownDocumentTreeNode;

fn find_children(
    available_collections: Arc<HashSet<String>>,
    parent: &MarkdownDocumentInCollection,
    sorted_documents: &mut LinkedList<MarkdownDocumentInCollection>,
) -> LinkedList<RhaiMarkdownDocumentTreeNode> {
    let children = sorted_documents
        .extract_if(|document| {
            document.collection_placement.parent == Some(parent.reference.basename())
        })
        .collect::<LinkedList<_>>();

    children
        .iter()
        .map(|document| RhaiMarkdownDocumentTreeNode {
            children: find_children(available_collections.clone(), document, sorted_documents),
            reference: RhaiMarkdownDocumentReference {
                front_matter: RhaiFrontMatter {
                    available_collections: available_collections.clone(),
                    front_matter: document.reference.front_matter.clone(),
                },
                reference: document.reference.clone(),
            },
        })
        .collect::<LinkedList<RhaiMarkdownDocumentTreeNode>>()
}

#[derive(Clone, Debug)]
pub struct RhaiMarkdownDocumentCollection {
    pub available_collections: Arc<HashSet<String>>,
    pub documents: Vec<MarkdownDocumentInCollection>,
}

impl RhaiMarkdownDocumentCollection {
    /// Returns a list of roots
    pub fn build_hierarchy(&self) -> Result<Vec<RhaiMarkdownDocumentTreeNode>> {
        let mut sorted_documents = self.sort_by_successors()?;
        let roots = sorted_documents
            .extract_if(|document| document.collection_placement.parent.is_none())
            .collect::<LinkedList<MarkdownDocumentInCollection>>();

        Ok(roots
            .iter()
            .map(|document| RhaiMarkdownDocumentTreeNode {
                children: find_children(
                    self.available_collections.clone(),
                    document,
                    &mut sorted_documents,
                ),
                reference: RhaiMarkdownDocumentReference {
                    front_matter: RhaiFrontMatter {
                        available_collections: self.available_collections.clone(),
                        front_matter: document.reference.front_matter.clone(),
                    },
                    reference: document.reference.clone(),
                },
            })
            .collect::<Vec<_>>())
    }

    pub fn sort_by_successors(&self) -> Result<LinkedList<MarkdownDocumentInCollection>> {
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
            if let Some(after) = &document.collection_placement.after {
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
                let mut sorted_documents: LinkedList<MarkdownDocumentInCollection> =
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

    fn rhai_hierarchy(
        &mut self,
    ) -> core::result::Result<RhaiMarkdownDocumentHierarchy, Box<EvalAltResult>> {
        match self.build_hierarchy() {
            Ok(hierarchy) => Ok(RhaiMarkdownDocumentHierarchy::from(hierarchy)),
            Err(err) => Err(format!("Unable to build hierarchy of documents: {err}").into()),
        }
    }
}

impl CustomType for RhaiMarkdownDocumentCollection {
    fn build(mut builder: TypeBuilder<Self>) {
        builder
            .with_name("RhaiMarkdownDocumentCollection")
            .with_get("hierarchy", Self::rhai_hierarchy);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::front_matter::FrontMatter;
    use crate::front_matter::collection_placement::CollectionPlacement;
    use crate::markdown_document_reference::MarkdownDocumentReference;

    fn create_document_reference(name: &str) -> MarkdownDocumentReference {
        MarkdownDocumentReference {
            basename_path: name.into(),
            front_matter: FrontMatter::mock(name),
            generated_page_base_path: "/".to_string(),
        }
    }

    #[test]
    fn test_sort_by_successors() -> Result<()> {
        let mut collection = RhaiMarkdownDocumentCollection {
            available_collections: Default::default(),
            documents: Default::default(),
        };

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: None,
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("1"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("5"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("4"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("1".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("2"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
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

    #[test]
    fn test_hierarchy() -> Result<()> {
        let mut collection = RhaiMarkdownDocumentCollection {
            available_collections: Default::default(),
            documents: Default::default(),
        };

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: None,
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("1"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: Some("3".to_string()),
            },
            reference: create_document_reference("5"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("3".to_string()),
                name: "my_collection".to_string(),
                parent: Some("3".to_string()),
            },
            reference: create_document_reference("4"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("1".to_string()),
                name: "my_collection".to_string(),
                parent: None,
            },
            reference: create_document_reference("2"),
        });

        collection.documents.push(MarkdownDocumentInCollection {
            collection_placement: CollectionPlacement {
                after: Some("2".to_string()),
                name: "my_collection".to_string(),
                parent: Some("1".to_string()),
            },
            reference: create_document_reference("3"),
        });

        let hierarchy = collection.build_hierarchy()?;

        // println!("hierarchy: {hierarchy:#?}");
        // assert!(false);

        let sorted: Vec<String> = hierarchy
            .iter()
            .map(|node| node.reference.front_matter.front_matter.title.clone())
            .collect();

        assert_eq!(sorted, ["1", "2"]);

        Ok(())
    }
}
