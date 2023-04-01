use std::sync::Arc;

use tokio::sync::Mutex;
use tower_http::add_extension::AddExtensionLayer;


pub struct DatasourceCombMutexLayer;

impl DatasourceCombMutexLayer {
    pub fn new() -> AddExtensionLayer<Arc<Mutex<()>>> {
        AddExtensionLayer::new(Arc::new(Mutex::new(())))
    }
}