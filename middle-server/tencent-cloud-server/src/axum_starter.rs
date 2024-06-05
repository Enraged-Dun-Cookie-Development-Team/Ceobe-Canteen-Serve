use std::sync::Arc;

use axum_starter::prepare;
use axum_starter::state::AddState;
use secrecy::SecretString;

use crate::config::TencentConfigTrait;
use crate::cloud_manager::PartCloudManagerState;


#[prepare(TencentCdnPrepare)]
pub async fn init_tencent_cdn<C>(config: &C) -> AddState<PartCloudManagerState>
where
    C: TencentConfigTrait,
{
    let manager = PartCloudManagerState::new(
        Arc::new(SecretString::new(config.get_secret_id().to_string())),
        Arc::new(SecretString::new(config.get_secret_key().to_string())),
    );

    AddState(manager)
}
