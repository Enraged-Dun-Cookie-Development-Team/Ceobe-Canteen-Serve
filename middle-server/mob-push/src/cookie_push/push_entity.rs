use mob_push::{
    push_notify::{
        android::{AndroidNotify, Image, NotifyStyle},
        ios::{IosNotify, IosRichTextType},
    },
    PushEntity,
};
use sql_models::{fetcher::datasource_config::models::model_datasource_config::NewCookiePushInfo, sql_connection::sea_orm::prelude::Uuid};

pub struct CookiePushEntity {
    avatar: String,
    datasource_name: String,
    image: Option<String>,
    content: String,
    identify: Uuid,
}

impl CookiePushEntity {
    pub fn new(
        datasource: NewCookiePushInfo,
        image: Option<String>,
        content: String,
    ) -> Self {
        let content = if content.chars().count() <= 50 {
            content
        } else {
            let mut content = content.chars().take(50).collect::<String>();
            content.push_str("...");
            content
        };

        Self {
            avatar: datasource.avatar,
            datasource_name: datasource.nickname,
            image,
            content,
            identify: datasource.unique_id,
        }
    }
}

impl PushEntity for CookiePushEntity {
    type Resource = Uuid;

    fn get_resource(&self) -> &Self::Resource {
        &self.identify
    }

    type Content = str;

    fn get_send_content(&self) -> &Self::Content {
        &self.content
    }

    fn android_notify(&self, notify: &mut AndroidNotify) {
        notify.set_image(Image::Image(self.avatar.clone()));
        if let Some(img) = &self.image {
            notify.set_notify_style(NotifyStyle::new_big_vision(
                img.to_owned(),
            ));
        }
    }

    fn ios_notify(&self, notify: &mut IosNotify) {
        if let Some(img) = &self.image {
            notify.set_rich_text(IosRichTextType::Picture(img.to_owned()));
        }
    }

    fn get_title(&self) -> std::borrow::Cow<'_, str> {
        format!("小刻在`{}`发现了新饼", self.datasource_name).into()
    }

    fn push_forward(&self, _push_forward: &mut mob_push::PushForward) {}
}
