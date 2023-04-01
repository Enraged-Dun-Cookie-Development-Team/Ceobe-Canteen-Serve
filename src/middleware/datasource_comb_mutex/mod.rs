use std::sync::Arc;

use tokio::sync::Mutex;
use tower::Layer;
use tower_http::add_extension::{AddExtensionLayer, AddExtension};

#[derive(Clone, Copy, Debug)]
pub struct DatasourceCombMutexLayer;

impl<S> Layer<S> for DatasourceCombMutexLayer
{
    type Service = AddExtension<S, Arc<Mutex<()>>>;

    fn layer(&self, inner: S) -> Self::Service {
        AddExtension::new(inner, Arc::new(Mutex::new(())))
    }
}