mod deserialize;
mod serialize;

use serde::Deserialize;
use serde::Serialize;

pub use self::deserialize::deserialize;
pub use self::serialize::serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct ListResourcesCursor {
    pub offset: i32,
}
