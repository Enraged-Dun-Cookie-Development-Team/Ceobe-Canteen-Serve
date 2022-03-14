#![feature(type_alias_impl_trait)]
#![feature(try_trait_v2)]
pub mod rr_try;

mod r_result;
mod serde;
mod to_response;
pub mod trans;
#[macro_use]
mod marco;
mod into_rresult;

pub use crate::serde::IntoSerde;
pub use crate::serde::Wrap;
pub use r_result::RResult;

pub use crate::into_rresult::{IntoRResult, IntoRResultWithCodeError};
