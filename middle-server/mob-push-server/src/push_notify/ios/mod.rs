pub mod content_avaliable;
pub mod subtitle;
use typed_builder::TypedBuilder;

use self::{apn::Category, content_avaliable::ContentAvailable, subtitle::Subtitle};
pub use self::{apn::IosPushSound, badge::IosBadgeType, rich_text::IosRichTextType};

use super::{NotifySerialize, SerializeInformation};

mod apn;
mod badge;
mod rich_text;

#[derive(Debug, TypedBuilder, Default, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct IosNotify {
    badge: Option<IosBadgeType>,
    category: Option<Category>,
    sound: Option<IosPushSound>,
    subtitle: Option<Subtitle>,
    content_available: Option<ContentAvailable>,
    rich_text: Option<IosRichTextType>,
}

impl NotifySerialize for IosNotify {
    fn serialize_field(&self) -> usize {
        self.badge.serialize_field()
            + self.category.serialize_field()
            + self.sound.serialize_field()
            + self.subtitle.serialize_field()
            + self.content_available.serialize_field()
            + self.rich_text.serialize_field()
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        self.badge.serialize::<S>(struct_serialize)?;
        self.category.serialize::<S>(struct_serialize)?;
        NotifySerialize::serialize::<S>(&self.sound, struct_serialize)?;
        self.subtitle.serialize::<S>(struct_serialize)?;
        self.content_available.serialize::<S>(struct_serialize)?;
        self.rich_text.serialize::<S>(struct_serialize)?;
        Ok(())
    }
}

impl SerializeInformation for IosNotify {
    fn serialize_name() -> &'static str {
        "iosNotify"
    }
}

impl IosNotify {
    pub fn set_badge(&mut self, badge: IosBadgeType) -> &mut Self {
        self.badge.replace(badge);
        self
    }
    pub fn set_category(&mut self, category: Category) -> &mut Self {
        self.category.replace(category);
        self
    }
    pub fn set_sound(&mut self, sound: IosPushSound) -> &mut Self {
        self.sound.replace(sound);
        self
    }
    pub fn set_subtitle(&mut self, subtitle: Subtitle) -> &mut Self {
        self.subtitle.replace(subtitle);
        self
    }
    pub fn set_content_available(
        &mut self,
        content_available: Option<ContentAvailable>,
    ) -> &mut Self {
        self.content_available = content_available;
        self
    }
    pub fn set_rich_text(&mut self, rich_text: IosRichTextType) -> &mut Self {
        self.rich_text.replace(rich_text);
        self
    }
}

#[cfg(test)]
mod test {

    use crate::push_notify::SerializeInformation;

    use super::{content_avaliable::ContentAvailable, IosNotify};

    #[test]
    fn test() {
        let mut notify = IosNotify::default().into_notify();

        notify
            .set_subtitle("Test Sub Title".into())
            .set_content_available(Some(ContentAvailable))
            .set_badge(super::IosBadgeType::Adding(12))
            .set_sound(super::IosPushSound::Custom("123456".into()))
            .set_rich_text(super::IosRichTextType::Picture("12123".into()));

        let out = serde_json::to_string_pretty(&notify).unwrap();

        println!("{out}")
    }
}
