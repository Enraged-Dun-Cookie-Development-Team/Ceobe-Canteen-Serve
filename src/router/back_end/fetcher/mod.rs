use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::{
    middleware::authorize::AuthorizeLayer, router::ServerRoute,
    utils::user_authorize::auth_level::prefabs::Chef,
};
pub struct FetcherConfigControllers;

pub fn fetcher_config() -> ServerRoute {
    Router::new()
        .route(
            "/uploadAvatar",
            post(FetcherConfigControllers::upload_avatar),
        )
        .route(
            "/platformList",
            get(FetcherConfigControllers::get_platform_list),
        )
        .route(
            "/createPlatform",
            post(FetcherConfigControllers::create_platform_config),
        )
        .route(
            "/updatePlatform",
            post(FetcherConfigControllers::update_platform_config),
        )
        .route(
            "/deletePlatform",
            delete(FetcherConfigControllers::delete_platform_config),
        )
        .route(
            "/getPlatformAndDatasourceOption",
            get(FetcherConfigControllers::get_platform_and_datasource_list),
        )
        .route(
            "/getDatasourceList",
            get(FetcherConfigControllers::get_datasource_list),
        )
        .route(
            "/createDatasource",
            post(FetcherConfigControllers::create_datasource_config),
        )
        .route(
            "/updateDatasource",
            post(FetcherConfigControllers::update_datasource_config),
        )
        .route(
            "/deleteDatasource",
            delete(FetcherConfigControllers::delete_datasource_config),
        )
        .route(
            "/uploadGlobalConfig",
            post(FetcherConfigControllers::upload_global_configs),
        )
        .route(
            "/getGlobalConfig",
            get(FetcherConfigControllers::get_global_configs),
        )
        .route(
            "/allPlatformList",
            get(FetcherConfigControllers::get_platform_all_list_with_basic_info),
        )
        .route(
            "/getAllDatasourceList",
            get(FetcherConfigControllers::get_datasource_name_list),
        )
        .route(
            "/getFetcherLiveNumber",
            get(FetcherConfigControllers::get_fetcher_max_live_number),
        )
        .route(
            "/uploadFetcherConfig",
            post(FetcherConfigControllers::upload_fetchers_configs),
        )
        .route(
            "/getFetcherConfigList",
            get(FetcherConfigControllers::get_fetchers_configs),
        )
        .route_layer(AuthorizeLayer::<Chef>::new())
}
