use std::collections::HashMap;

use petgraph::algo::toposort;
use petgraph::stable_graph::NodeIndex;
use petgraph::stable_graph::StableDiGraph;

use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_in_collection::ContentDocumentInCollection;
use crate::content_document_tree_node::ContentDocumentTreeNode;
use crate::content_error::ContentError;

#[derive(Clone, Debug, Default)]
pub struct ContentDocumentCollection {
    pub documents: Vec<ContentDocumentInCollection>,
    pub name: String,
}

impl ContentDocumentCollection {
    pub fn build_hierarchy(&self) -> Result<Vec<ContentDocumentTreeNode>, ContentError> {
        let sorted_documents = self.sort_by_successors()?;

        Ok(self.child_nodes(&sorted_documents, None))
    }

    fn child_nodes(
        &self,
        sorted_documents: &[ContentDocumentInCollection],
        parent_basename: Option<&ContentDocumentBasename>,
    ) -> Vec<ContentDocumentTreeNode> {
        sorted_documents
            .iter()
            .filter(|document| document.collection_placement.parent.as_ref() == parent_basename)
            .map(|document| ContentDocumentTreeNode {
                children: self.child_nodes(sorted_documents, Some(&document.reference.basename())),
                collection_name: self.name.clone(),
                reference: document.reference.clone(),
            })
            .collect()
    }

    fn sort_by_successors(&self) -> Result<Vec<ContentDocumentInCollection>, ContentError> {
        let mut successors_graph: StableDiGraph<&ContentDocumentInCollection, ()> =
            StableDiGraph::new();
        let document_nodes: Vec<NodeIndex> = self
            .documents
            .iter()
            .map(|document| successors_graph.add_node(document))
            .collect();
        let node_by_basename: HashMap<ContentDocumentBasename, NodeIndex> = self
            .documents
            .iter()
            .zip(&document_nodes)
            .map(|(document, document_node)| (document.reference.basename(), *document_node))
            .collect();

        for (document, document_node) in self.documents.iter().zip(&document_nodes) {
            if let Some(after) = &document.collection_placement.after {
                let after_node = node_by_basename.get(after).ok_or_else(|| {
                    ContentError::AfterDocumentNotInCollection {
                        after: after.clone(),
                        basename: document.reference.basename(),
                        collection_name: self.name.clone(),
                    }
                })?;

                successors_graph.add_edge(*after_node, *document_node, ());
            }
        }

        toposort(&successors_graph, None)
            .map(|sorted_nodes| {
                sorted_nodes
                    .into_iter()
                    .map(|sorted_node| successors_graph[sorted_node].clone())
                    .collect()
            })
            .map_err(|cycle| ContentError::CollectionCycle {
                basename: successors_graph[cycle.node_id()].reference.basename(),
                collection_name: self.name.clone(),
            })
    }
}
