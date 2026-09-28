use std::collections::HashMap;

use poet_mdx::document_error_collection::DocumentErrorCollection;

use crate::collection_placement::CollectionPlacement;
use crate::content_document_basename::ContentDocumentBasename;
use crate::content_document_collection::ContentDocumentCollection;
use crate::content_document_collection_ranked::ContentDocumentCollectionRanked;
use crate::content_document_in_collection::ContentDocumentInCollection;
use crate::content_document_reference::ContentDocumentReference;
use crate::content_document_source::ContentDocumentSource;
use crate::content_error::ContentError;

fn validate_primary_collection(
    reference: &ContentDocumentReference,
    document_errors: &DocumentErrorCollection<ContentError>,
) {
    if let Some(primary_collection) = &reference.front_matter.primary_collection
        && !reference
            .front_matter
            .collections
            .iter()
            .any(|collection_placement| collection_placement.name == *primary_collection)
    {
        document_errors.register_error(
            reference.basename().to_string(),
            ContentError::PrimaryCollectionNotAmongPlacements {
                basename: reference.basename(),
                primary_collection: primary_collection.clone(),
            },
        );
    }
}

pub struct ContentDocumentIndex {
    pub content_document_basename_by_id: HashMap<String, ContentDocumentBasename>,
    pub content_document_by_basename: HashMap<ContentDocumentBasename, ContentDocumentReference>,
    pub content_document_collections: HashMap<String, ContentDocumentCollection>,
}

impl ContentDocumentIndex {
    pub fn build(content_document_sources: &[ContentDocumentSource]) -> Result<Self, ContentError> {
        let mut content_document_index = Self {
            content_document_basename_by_id: HashMap::new(),
            content_document_by_basename: content_document_sources
                .iter()
                .map(|ContentDocumentSource { reference, .. }| {
                    (reference.basename(), reference.clone())
                })
                .collect(),
            content_document_collections: HashMap::new(),
        };
        let document_errors = DocumentErrorCollection::default();

        for ContentDocumentSource { reference, .. } in content_document_sources {
            content_document_index.register_document_id(reference, &document_errors);
            validate_primary_collection(reference, &document_errors);

            for collection_placement in &reference.front_matter.collections {
                content_document_index.register_collection_placement(
                    collection_placement,
                    reference,
                    &document_errors,
                );
            }
        }

        if document_errors.is_empty() {
            Ok(content_document_index)
        } else {
            Err(ContentError::InvalidDocuments(document_errors))
        }
    }

    pub fn rank_collections(
        &self,
    ) -> Result<HashMap<String, ContentDocumentCollectionRanked>, ContentError> {
        self.content_document_collections
            .values()
            .map(|content_document_collection| {
                ContentDocumentCollectionRanked::try_from(content_document_collection).map(
                    |content_document_collection_ranked| {
                        (
                            content_document_collection_ranked.name.clone(),
                            content_document_collection_ranked,
                        )
                    },
                )
            })
            .collect()
    }

    fn register_collection_placement(
        &mut self,
        collection_placement: &CollectionPlacement,
        reference: &ContentDocumentReference,
        document_errors: &DocumentErrorCollection<ContentError>,
    ) {
        if let Some(after) = &collection_placement.after
            && !self.content_document_by_basename.contains_key(after)
        {
            document_errors.register_error(
                reference.basename().to_string(),
                ContentError::SucceedingDocumentNotFound {
                    after: after.clone(),
                    basename: reference.basename(),
                },
            );
        }

        if let Some(parent) = &collection_placement.parent
            && !self.content_document_by_basename.contains_key(parent)
        {
            document_errors.register_error(
                reference.basename().to_string(),
                ContentError::ParentDocumentNotFound {
                    basename: reference.basename(),
                    parent: parent.clone(),
                },
            );
        }

        self.content_document_collections
            .entry(collection_placement.name.clone())
            .or_insert_with(|| ContentDocumentCollection {
                documents: vec![],
                name: collection_placement.name.clone(),
            })
            .documents
            .push(ContentDocumentInCollection {
                collection_placement: collection_placement.clone(),
                reference: reference.clone(),
            });
    }

    fn register_document_id(
        &mut self,
        reference: &ContentDocumentReference,
        document_errors: &DocumentErrorCollection<ContentError>,
    ) {
        if let Some(id) = &reference.front_matter.id
            && let Some(first_basename) = self
                .content_document_basename_by_id
                .insert(id.clone(), reference.basename())
        {
            document_errors.register_error(
                reference.basename().to_string(),
                ContentError::DuplicateDocumentId {
                    first_basename,
                    id: id.clone(),
                    second_basename: reference.basename(),
                },
            );
        }
    }
}
