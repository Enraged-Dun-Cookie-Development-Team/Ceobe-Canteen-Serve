use ceobe_qiniu_upload::ObjectName;
use futures::{
    future::{ready, Ready},
    io::Cursor,
};
use mob_push_server::{
    push_notify::android::{Image, NotifyStyle},
    PushEntity,
};
use mongo_migration::mongo_models::mongodb::bson::oid::ObjectId;
use qiniu_cdn_upload::{
    update_payload::UploadPayload, update_source::UploadSource,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

// 分页饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListResp {
    pub cookies: Vec<Value>,
    pub next_page_id: Option<ObjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
#[serde(deny_unknown_fields)]
pub struct CookieListReq {
    pub datasource_comb_id: String,
    pub cookie_id: ObjectId,
}

#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieDatasourceReq {
    pub datasource: String,
    pub unique: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieContentReq {
    pub text: Option<String>,
    pub image_url: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct NewCookieReq {
    pub source: CookieDatasourceReq,
    pub content: CookieContentReq,
    pub cookie_id: ObjectId,
}

pub struct PushInfo {
    pub content: Option<String>,
    pub datasource_name: String,
    pub image_url: Option<String>,
    pub icon_url: String,
}

impl PushEntity for PushInfo {
    type Content = str;

    fn get_send_content(&self) -> &Self::Content {
        if let Some(content) = &self.content {
            content
        }
        else {
            ""
        }
    }

    fn get_title(&self) -> std::borrow::Cow<'_, str> {
        let name = &self.datasource_name;
        format!(r#"小刻在【{name}】找到了一个饼！！"#).into()
    }

    fn android_notify(
        &self,
        notify: &mut mob_push_server::push_notify::android::AndroidNotify,
    ) {
        if let Some(image) = &self.image_url {
            notify.set_notify_style(NotifyStyle::new_big_vision(image));
        }
        notify.set_image(Image::new_image(&self.icon_url));
    }
}

// TODO: 以下两段和其他地方重复了，之后抽到service层
pub struct DeleteObjectName {
    pub file_name: String,
}

impl<'s> ObjectName<'s> for DeleteObjectName {
    const DIR: Option<&'s str> = Some("datasource-comb");

    fn file_name(&self) -> &str { &self.file_name }
}
/// 数据源组合id-最新饼id 上传对象储存
#[derive(Debug, Clone, Serialize)]
pub struct CombIdToCookieId {
    pub cookie_id: Option<ObjectId>,
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
    type Source<'r> = &'r CombIdToCookieId;

    fn read_data(payload: Self::Source<'_>) -> Self::ReadFuture<'_> {
        ready(serde_json::to_vec(payload).map(Cursor::new))
    }

    fn content_type(
        _payload: &Self::Source<'_>,
    ) -> ceobe_qiniu_upload::mime_guess::Mime {
        "application/json; charset=utf-8".parse().unwrap()
    }
}
