use axum::{
    routing::{delete, get, post},
    Router,
};

use super::{CeobeOperationAuth, CeobeTools};
use crate::middleware::authorize::AuthorizeLayer;

pub struct CeobeOpToolLink;

pub fn tool_link_router() -> crate::router::ServerRoute {
    Router::new()
        .route("/uploadAvatar", post(CeobeOpToolLink::upload_avatar))
        .route("/pageShow", get(CeobeOpToolLink::all_with_paginator))
        .route("/createOne", post(CeobeOpToolLink::create_one_mongo))
        .route("/updateOne", post(CeobeOpToolLink::update_one_mongo))
        .route("/deleteOne", delete(CeobeOpToolLink::delete_one_mongo))
        .route_layer(AuthorizeLayer::<CeobeTools>::new())
        .route("/create", post(CeobeOpToolLink::create_one))
        .route("/update", post(CeobeOpToolLink::update_one))
        .route("/delete", delete(CeobeOpToolLink::delete_one))
        .route("/list", get(CeobeOpToolLink::list))
}
