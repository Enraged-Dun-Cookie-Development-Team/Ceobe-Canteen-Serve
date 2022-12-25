pub mod platform_configs;
pub mod global_configs;
use resp_result::RespResult;

use self::error::FetcherError;

type FetcherResult<T> = RespResult<T, FetcherError>;
mod controller {
    use axum::extract::{multipart::MultipartRejection, Multipart};
    use ceobe_qiniu_upload::QiniuUploader;
    use qiniu_cdn_upload::upload;

    use super::{
        error::FieldNotExist, utils::DataSourceAvatarPayload, view::AvatarId,
        FetcherResult,
    };
    use crate::router::FetcherConfigControllers;

    impl FetcherConfigControllers {
        pub async fn upload_avatar(
            uploader: QiniuUploader,
            multipart: Result<Multipart, MultipartRejection>,
        ) -> FetcherResult<AvatarId> {
            resp_result::resp_try(async move {
                let mut multipart = multipart?;
                let field =
                    multipart.next_field().await?.ok_or(FieldNotExist)?;

                let resp =
                    upload(&uploader, field, DataSourceAvatarPayload::new())
                        .await
                        .map(Into::into)?;

                Ok(resp)
            })
            .await
        }
    }
}

mod view {
    use ceobe_qiniu_upload::ResponsePayload;
    use serde::Serialize;

    #[derive(Debug, Serialize)]
    pub struct AvatarId {
        path: String,
    }

    impl From<ResponsePayload> for AvatarId {
        fn from(ResponsePayload { key, .. }: ResponsePayload) -> Self {
            Self { path: key }
        }
    }
}

mod error {
    use axum::extract::multipart::{MultipartError, MultipartRejection};
    use ceobe_qiniu_upload::Error as QiniuError;
    use status_err::{ErrPrefix, StatusErr};

    use crate::error_generate;
    error_generate! {
        pub FetcherError

        Upload = QiniuError
        Multipart = MultipartError
        MultipartReject = MultipartRejection
        Field = FieldNotExist
    }

    #[derive(Debug, thiserror::Error)]
    #[error("Field 不存在")]
    pub struct FieldNotExist;

    impl StatusErr for FieldNotExist {
        fn prefix(&self) -> status_err::ErrPrefix { ErrPrefix::CHECKER }

        fn code(&self) -> u16 { 0x0011 }
    }
}

mod utils {
    use qiniu_cdn_upload::{
        update_payload::UploadPayload, update_source::FieldSource,
    };
    use uuid::Uuid;

    pub struct DataSourceAvatarPayload(String);

    impl DataSourceAvatarPayload {
        pub fn new() -> Self { Self(Uuid::new_v4().to_string()) }
    }

    impl UploadPayload for DataSourceAvatarPayload {
        type Source = FieldSource;

        const DIR: &'static str = "data-source-avatar";

        fn obj_name(&self) -> &str { &self.0 }
    }
}
