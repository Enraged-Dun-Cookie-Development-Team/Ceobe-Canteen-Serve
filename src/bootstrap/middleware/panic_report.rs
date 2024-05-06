use axum::response::IntoResponse;
use axum_starter::{prepare, PrepareMiddlewareEffect};
use persistence::operate::FromRequestParts;
use qq_channel_warning::{
    qq_channel_logger, GrpcConfigTrait, LogRequest, LogType,
    QqChannelGrpcService,
};
use resp_result::RespResult;
use tower_http::catch_panic::{CatchPanicLayer, ResponseForPanic};
use tracing::{error, instrument};

use crate::error::ServicePanic;

#[prepare(PrepareCatchPanic?)]
pub async fn prepare_catch_panic<C: GrpcConfigTrait>(
    cfg: &C,
) -> Result<PanicReport, qq_channel_warning::Error> {
    let add = qq_channel_logger(cfg).0;

    let (mut part, _) = http::Request::new(()).into_parts();
    let ret = QqChannelGrpcService::from_request_parts(&mut part, &add).await;
    let service = match ret {
        Ok(service) => service,
        Err(RespResult::Err(err)) => Err(err)?,
        _ => unreachable!(),
    };

    Ok(PanicReport(service))
}

pub struct PanicReport(QqChannelGrpcService);

impl<S> PrepareMiddlewareEffect<S> for PanicReport {
    type Middleware = CatchPanicLayer<PanicReportHandle>;

    fn take(self, _: &mut axum_starter::StateCollector) -> Self::Middleware {
        CatchPanicLayer::custom(PanicReportHandle(self.0))
    }
}

#[derive(Debug, Clone)]
pub struct PanicReportHandle(QqChannelGrpcService);

impl ResponseForPanic for PanicReportHandle {
    type ResponseBody = axum::body::Body;

    #[instrument(skip_all)]
    fn response_for_panic(
        &mut self, err: Box<dyn std::any::Any + Send + 'static>,
    ) -> http::Response<Self::ResponseBody> {
        // TODO: 推送panic到频道好像有问题
        let err = if let Some(msg) = err
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| err.downcast_ref::<&str>().copied())
        {
            msg
        }
        else {
            "Unknown Panic Message"
        }
        .to_owned();
        error!(unexpectedPanic.detail = err);
        let service = self.0.clone();
        tokio::spawn(async move {
            let mut service = service;
            service
                .send_logger(
                    LogRequest::builder()
                        .level(LogType::Error)
                        .info("Rust Service 发现Panic!".into())
                        .extra(err)
                        .build(),
                )
                .await
                .ok();
        });
        RespResult::<(), _>::err(ServicePanic).into_response()
    }
}
