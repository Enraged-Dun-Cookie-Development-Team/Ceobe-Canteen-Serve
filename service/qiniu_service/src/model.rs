use std::future::{ready, Ready};

use ceobe_qiniu_upload::ObjectName;
use futures::io::Cursor;
use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::UploadSource,
};
use serde::Serialize;

pub struct DeleteObjectName {
    pub file_name: String,
}

impl<'s> ObjectName<'s> for DeleteObjectName {
    const DIR: Option<&'s str> = Some("datasource-comb");

    fn file_name(&self) -> &str { &self.file_name }
}
/// 数据源组合id-最新饼id 上传对象储存
#[derive(Debug, Clone, Serialize)]
pub struct CombIdToCookieId<'s> {
    /// 最新饼id
    pub cookie_id: Option<&'s str>,
    /// 后更新的饼id
    pub update_cookie_id: Option<&'s str>,
}
#[derive(Debug, Clone, Copy)]
pub struct CombIdToCookieIdPlayLoad<'s> {
    pub file_name: &'s str,
}
pub struct CombIdToCookieSource;

impl<'s> UploadPayload for CombIdToCookieIdPlayLoad<'s> {
    type Source = CombIdToCookieSource;

    const DIR: &'static str = "datasource-comb";

    fn obj_name(&self) -> &str { self.file_name }
}

impl UploadSource for CombIdToCookieSource {
    type Error = serde_json::error::Error;
    type Read = Cursor<Vec<u8>>;
    type ReadFuture<'f> = Ready<Result<Self::Read, Self::Error>>;
    type Source<'r> = &'r CombIdToCookieId<'r>;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_> {
        ready(serde_json::to_vec(payload).map(Cursor::new))
    }

    fn content_type(
        _payload: &Self::Source<'_>,
    ) -> ceobe_qiniu_upload::mime_guess::Mime {
        "application/json; charset=utf-8".parse().unwrap()
    }
}
