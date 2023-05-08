use axum_starter::{prepare, state::AddState};
use tonic::transport::{Channel, Endpoint};

use crate::config::GrpcConfigTrait;

#[derive(Debug, Clone)]
pub struct QqChannelGrpcState {
    pub(crate) uri: Endpoint,
}

#[prepare(QqChannelPrepare 'arg)]
pub fn qq_channel_logger<'arg, C: GrpcConfigTrait>(
    cfg: &'arg C,
) -> AddState<QqChannelGrpcState> {

    AddState(QqChannelGrpcState {
        uri: Channel::builder(cfg.get_uri()),
    })
}
