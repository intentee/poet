use std::sync::Arc;

use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;
use crate::holder::Holder;

pub struct AppData {
    pub filesystem_http_route_index_holder: Holder<Arc<FilesystemHttpRouteIndex>>,
}
