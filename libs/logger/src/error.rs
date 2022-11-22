use std::io;

use tracing_subscriber::util::TryInitError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    TracingInit(#[from] TryInitError),
}
