use super::model_platform_config::{Model, PlatformHasDatasource};

impl PlatformHasDatasource {
    pub fn from_model(model: Model, has_datasource: bool) -> Self {
        Self {
            id: model.id,
            type_id: model.type_id,
            platform_name: model.platform_name,
            min_request_interval: model.min_request_interval,
            has_datasource,
        }
    }
}
