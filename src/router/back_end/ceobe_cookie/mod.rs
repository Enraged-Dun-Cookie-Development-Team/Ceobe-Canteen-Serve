use axum::Router;
pub use newest::CeobeCookieNewestBackend;

use self::newest::newest_router;
use crate::{
    middleware::authorize::AuthorizeLayer, new_auth_level,
    utils::user_authorize::auth_level::prefabs::Chef,
};

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
