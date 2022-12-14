mod back_end;
mod front_end;

use std::{marker::PhantomData, pin::Pin};

use axum::{
    body::Bytes,
    extract::{
        multipart::{Field, MultipartError},
        Multipart,
    },
    routing::{get, post},
    Router,
};
pub use back_end::{
    BakeryMansionBackend, CeobeOpResource, CeobeOpVersion,
    CeobeOperationAnnouncement, CeobeOperationVideo, UserAuthBackend,
};
use ceobe_qiniu_upload::{
    mime::APPLICATION_OCTET_STREAM, mime_guess::Mime, QiniuUploader,
    ResponsePayload,
};
pub use front_end::{
    BakeryMansionFrontend, CeobeOperationAnnouncementFrontend,
    CeobeOperationResourceFrontend, CeobeOperationVersionFrontend,
    CeobeOperationVideoFrontend,
};
use futures::{io::Cursor, Future};
use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::UploadSource,
};

pub type ServerRoute = Router<State>;

use crate::bootstrap::init::State;

use self::{back_end::back_end_router, front_end::front_end_router};

pub fn root_route() -> ServerRoute {
    Router::new()
        .nest("/canteen", front_end_router())
        .nest("/admin", back_end_router())
        .route(
            "/panic",
            get(|| async {
                #[cfg(debug_assertions)]
                {
                    panic!("测试 Panic");
                }
                #[cfg(not(debug_assertions))]
                resp_result::RespResult::<_, crate::error::NotAnError>::ok(
                    "不可以Panic",
                )
            }),
        )
        .route("/upload", post(upload))
}

async fn upload(
    uploader: QiniuUploader, mut file: Multipart,
) -> Result<axum::Json<ResponsePayload>, String> {
    let source = file
        .next_field()
        .await
        .map_err(|err| format!("load field error {:?}", err))?
        .ok_or("No field".to_string())?;

    let v = qiniu_cdn_upload::upload(
        &uploader,
        source,
        ImagePayload::<ImageSource>(PhantomData, "AAA".to_string()),
    )
    .await
    .map_err(|err| format!("{:?}", err))?;

    Ok(axum::Json(v))
}

struct ImagePayload<Source>(PhantomData<Source>, String);

impl<Source> UploadPayload for ImagePayload<Source>
where
    Source: UploadSource + 'static,
{
    type Source = Source;

    const DIR: &'static str = "image";

    fn obj_name(&self) -> &str {
        self.1.as_str()
    }
}

struct ImageSource;

impl qiniu_cdn_upload::update_source::UploadSource for ImageSource {
    type Source<'r> = Field<'r>;

    type Read = Cursor<Bytes>;

    type Error = MultipartError;

    type ReadFuture<'f> = Pin<
        Box<dyn Future<Output = Result<Self::Read, Self::Error>> + Send + 'f>,
    >;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_> {
        Box::pin(async move {
            let body = payload.bytes().await?;
            Ok(Cursor::new(body))
        })
    }

    fn content_type(payload: &Self::Source<'_>) -> Mime {
        payload
            .content_type()
            .and_then(|v| v.parse().ok())
            .unwrap_or(APPLICATION_OCTET_STREAM)
    }
}
