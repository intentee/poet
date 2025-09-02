use serde::Deserialize;
use serde::Serialize;

use crate::front_matter::collection_placement::CollectionPlacement;

#[derive(Clone, Default, Debug, Deserialize, Serialize)]
#[serde(transparent)]
pub struct CollectionPlacementList {
    pub placements: Vec<CollectionPlacement>,
}
