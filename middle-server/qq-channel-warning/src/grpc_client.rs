use axum::extract::{FromRef, FromRequestParts};
use resp_result::RespResult;
use tonic::transport::Channel;

use crate::{
    axum_starter::QqChannelGrpcState,
    error,
    proto_reexport::{LogClient, LogRequest},
};
#[derive(Debug, Clone)]
pub struct QqChannelGrpcService {
    client: LogClient<Channel>,
}

impl<S> FromRequestParts<S> for QqChannelGrpcService
where
    S: Send,
    QqChannelGrpcState: FromRef<S>,
{
    type Rejection = RespResult<resp_result::Nil, error::Error>;

    fn from_request_parts<'life0, 'life1, 'async_trait>(
        _parts: &'life0 mut axum::http::request::Parts,
        state: &'life1 S,
    ) -> core::pin::Pin<
        Box<
            dyn core::future::Future<Output = Result<Self, Self::Rejection>>
                + core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        let uri = QqChannelGrpcState::from_ref(state).uri;
        Box::pin(async move {
            LogClient::connect(uri)
                .await
                .map_err(error::Error::Transport)
                .map_err(RespResult::Err)
                .map(|client| Self { client })
        })
    }
}

impl QqChannelGrpcService {
    /// send logger info to qq channel by the Grpc service
    pub async fn send_logger(
        &mut self,
        log_info: LogRequest,
    ) -> Result<(), error::Error> {
        let resp = self.client.push_log(log_info).await?;
        if !resp.into_inner().success {
            Err(error::Error::PushLogFailure)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::QqChannelGrpcService;
    use crate::LogRequest;
    use crate::{proto_reexport::LogClient, LogType};
    use tonic::transport::Channel;
    #[tokio::test]
    async fn test_send() {
        let channel = Channel::from_shared("http://127.0.0.1:8003").expect("Bad URL");
        let client = LogClient::connect(channel)
            .await
            .expect("connect to grpc service failure");

        let mut server = QqChannelGrpcService { client };

         server
            .send_logger(
                LogRequest::builder()
                    .level(LogType::Info)
                    .info(String::from("测试 测试"))
                    .manual()
                    .extra(format!("[{}:{}/{}]",module_path!(),line!(),column!()))
                    .build(),
            )
            .await
            .expect("send failure");
    }
}
