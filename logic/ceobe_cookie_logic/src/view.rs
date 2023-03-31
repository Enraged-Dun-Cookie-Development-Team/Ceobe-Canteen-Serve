use mob_push_server::{
    push_notify::android::{Image, NotifyStyle},
    PushEntity,
};
use mongo_migration::mongo_models::mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use typed_builder::TypedBuilder;

// 分页饼列表返回模型
#[derive(Debug, Clone, Serialize, Deserialize, TypedBuilder)]
pub struct CookieListResp {
    pub cookies: Vec<Value>,
    pub next_page_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, TypedBuilder)]
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
#[derive(Debug, Clone, Deserialize, TypedBuilder)]
pub struct NewCookieReq {
    pub source: CookieDatasourceReq,
    pub content: CookieContentReq,
    pub cookie_id: ObjectId,
}

#[derive(Debug, Clone, TypedBuilder)]
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
            " "
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
