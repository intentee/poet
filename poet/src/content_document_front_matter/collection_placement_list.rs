use serde::Deserialize;
use serde::Serialize;

use crate::content_document_front_matter::collection_placement::CollectionPlacement;

#[derive(Clone, Default, Debug, Deserialize, Hash, Serialize)]
#[serde(transparent)]
pub struct CollectionPlacementList {
    pub placements: Vec<CollectionPlacement>,
}
