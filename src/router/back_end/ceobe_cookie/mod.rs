use axum::Router;

use crate::{new_auth_level, middleware::authorize::AuthorizeLayer, utils::user_authorize::auth_level::prefabs::Chef};

use self::newest::newest_router;

pub use newest::CeobeCookieNewestBackend;

mod newest;

pub(super) fn ceobe_cookie_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/newest", newest_router())
        .route_layer(AuthorizeLayer::<CeobeCookieAuth>::new())
}

new_auth_level! {
    pub CeobeCookieAuth=>[
        Chef
    ]
}