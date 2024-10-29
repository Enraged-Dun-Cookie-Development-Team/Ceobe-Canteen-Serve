mod controller_trait;
mod endpoint_type;
mod router_extra;
mod view_traits;

pub use axum;
pub use axum_resp_result;
pub use controller_trait::{
    ControllerError, ControllerRouter, HandlerMapReject, HandlerResult,
    LayeredController,
};
pub use endpoint_type::EndpointType;
pub use router_extra::ControllerRouterExt;
pub use status_err;
pub use thiserror::Error as ThisError;
pub use tracing;
pub use view_traits::{
    OptionField, OptionViewField, SkipField, ValueField,
};

pub mod endpoint {
    pub use crate::endpoint_type::{AdminEnd, Internal, UserEnd, CDN};
}
