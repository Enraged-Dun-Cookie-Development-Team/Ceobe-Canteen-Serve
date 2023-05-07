use axum::http::uri::InvalidUri;
use axum_starter::{prepare, state::AddState};
use tonic::transport::{Channel, Endpoint};

use crate::config::GrpcConfigTrait;

#[derive(Debug, Clone)]
pub struct QqChannelGrpcState {
    pub(crate) uri: Endpoint,
}

#[prepare(QqChannel? 'arg)]
pub fn qq_channel_logger<'arg, C: GrpcConfigTrait>(
    cfg: &'arg C,
) -> Result<AddState<QqChannelGrpcState>, InvalidUri> {
    Ok(AddState(QqChannelGrpcState {
        uri: Channel::from_shared(cfg.get_uri().to_string())?,
    }))
}
