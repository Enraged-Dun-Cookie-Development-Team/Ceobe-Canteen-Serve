pub mod resource_data;

use std::convert::Infallible;

use thiserror::Error;
pub use CheckError::*;

#[derive(Debug, Error)]
pub enum CheckError {}

impl From<Infallible> for CheckError {
    fn from(_: Infallible) -> Self { unreachable!() }
}
