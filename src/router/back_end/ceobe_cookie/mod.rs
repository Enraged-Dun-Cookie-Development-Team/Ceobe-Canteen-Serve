use authorize_server::{mix_role_gen, roles::Chef};
use axum::Router;
pub use newest::CeobeCookieNewestBackend;

use self::newest::newest_router;
use crate::middleware::authorize::AuthorizeLayer;

mod newest;

pub(super) fn ceobe_cookie_router() -> crate::router::ServerRoute {
    Router::new()
        .nest("/newest", newest_router())
        .route_layer(AuthorizeLayer::<CeobeCookieAuth>::new())
}

mix_role_gen! {
    pub CeobeCookieAuth=>[
        Chef
    ]
}
