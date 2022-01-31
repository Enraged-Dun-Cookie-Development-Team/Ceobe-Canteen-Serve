pub mod trans;
mod r_result;
mod serde;
mod to_response;
#[macro_use]
mod marco;

pub use r_result::RResult;
pub use crate::serde::Wrap;
pub use crate::serde::IntoSerde;