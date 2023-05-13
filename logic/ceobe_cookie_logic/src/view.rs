use ceobe_cookie::CookieTimestamp;
use db_ops_prelude::mongo_models::ceobe::cookie::analyze::models::{
    images::CookieImages, meta::Item,
};
use mob_push_server::{
    push_notify::android::{Image, NotifyStyle},
    PushEntity,
};
use mongo_migration::mongo_models::mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

// 分页临时饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieTempListResp {
    pub cookies: Vec<Value>,
    pub next_page_id: Option<String>,
}

// 分页临时饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListResp {
    pub cookies: Vec<SingleCookie>,
    pub next_page_id: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct SingleCookie {
    // 数据源名字
    pub datasource: String,
    // 数据源icon
    pub icon: String,
    pub timestamp: CookieTimestamp,
    pub default_cookie: DefaultCookie,
    pub item: Item,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct DefaultCookie {
    pub text: String,
    pub images: Option<Vec<CookieImages>>,
}

// 饼列表请求
#[derive(Debug, Clone, Deserialize, TypedBuilder)]
#[serde(deny_unknown_fields)]
pub struct CookieListReq {
    pub datasource_comb_id: String,
    pub cookie_id: ObjectId,
    pub update_cookie_id: Option<ObjectId>,
}

// 从分析器来的新饼信息
#[derive(Debug, Clone, Deserialize, TypedBuilder)]
pub struct NewCookieReq {
    pub source: CookieDatasourceReq,
    pub content: CookieContentReq,
    pub cookie_id: ObjectId,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieDatasourceReq {
    pub datasource: String,
    pub unique: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieContentReq {
    #[serde(deserialize_with = "empty_change_to_space")]
    pub text: String,
    pub image_url: Option<String>,
}

fn empty_change_to_space<'de, D: Deserializer<'de>>(
    d: D,
) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let value: String = String::deserialize(d)?;
    Ok(match value.is_empty() {
        true => " ".to_owned(),
        _ => value.to_owned(),
    })
}

// app推送信息
#[derive(Debug, Clone, TypedBuilder)]
pub struct PushInfo {
    pub content: String,
    pub datasource_name: String,
    pub image_url: Option<String>,
    pub icon_url: String,
}
impl PushEntity for PushInfo {
    type Content = str;

    fn get_send_content(&self) -> &Self::Content { &self.content }

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
