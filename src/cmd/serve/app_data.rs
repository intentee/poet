use std::sync::Arc;

use crate::filesystem_http_route_index::FilesystemHttpRouteIndex;

pub struct AppData {
    pub filesystem_http_route_index: Arc<FilesystemHttpRouteIndex>,
}
