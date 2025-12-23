mod deserialize;
mod serialize;

use serde::Deserialize;
use serde::Serialize;

pub use self::deserialize::deserialize;
pub use self::serialize::serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResourcesCursor {
    pub offset: usize,
    pub per_page: usize,
}

impl Default for ListResourcesCursor {
    fn default() -> Self {
        Self {
            offset: 0,
            per_page: 20,
        }
    }
}
