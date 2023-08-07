use axum_starter::{prepare, state::AddState};
use tonic::transport::{Channel, Endpoint};

use crate::config::GrpcConfigTrait;

#[derive(Debug, Clone)]
pub struct QqChannelGrpcState {
    pub(crate) uri: Endpoint,
}

#[prepare(origin QqChannelPrepare)]
pub fn qq_channel_logger<C: GrpcConfigTrait>(
    cfg: &C,
) -> AddState<QqChannelGrpcState> {
    AddState(QqChannelGrpcState {
        uri: Channel::builder(cfg.get_uri()),
    })
}
