use std::sync::Arc;

use axum_starter::{prepare, state::AddState};
use secrecy::SecretString;

use crate::{
   config::TencentConfigTrait,
};
pub use crate::cloud_manager::PartTencentCloudManagerState;

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
