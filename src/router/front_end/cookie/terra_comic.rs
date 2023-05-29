use axum::{routing::get, Router};

use crate::router::ServerRoute;

pub struct CookieTerraComicFrontend;

pub(super) fn terra_comic_router() -> ServerRoute {
    Router::new()
        .route("/list", get(CookieTerraComicFrontend::comic_list))
        .route("/episodeList", get(CookieTerraComicFrontend::comic_episode_list))
}