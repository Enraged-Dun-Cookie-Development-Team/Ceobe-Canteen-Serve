use std::sync::Arc;

use axum_starter::{prepare, state::AddState};
pub use general_request_client::client::RequestClient;
use secrecy::SecretString;

pub use crate::cloud_manager::PartTencentCloudManagerState;
use crate::config::TencentConfigTrait;

#[prepare(TencentCloudPrepare)]
pub async fn init_tencent_cdn<C>(
    config: &C,
) -> AddState<PartTencentCloudManagerState>
where
    C: TencentConfigTrait,
{
    let manager = PartTencentCloudManagerState::new(
        Arc::new(SecretString::new(config.get_secret_id().to_string())),
        Arc::new(SecretString::new(config.get_secret_key().to_string())),
        Arc::new(config.get_cdn_base_url()),
    );

    AddState(manager)
}
