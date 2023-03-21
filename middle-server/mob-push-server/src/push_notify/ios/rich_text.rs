use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]
pub enum IosRichTextType {
    None,
    Picture(String),
    Video(String),
    Voice(String),
}

impl NotifySerialize for IosRichTextType {
    fn serialize_field(&self) -> usize {
        match self {
            IosRichTextType::None => 1,
            IosRichTextType::Picture(_) | IosRichTextType::Video(_) | IosRichTextType::Voice(_) => {
                3
            }
        }
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        let enable = if let IosRichTextType::None = self {
            0
        } else {
            1
        };
        struct_serialize.serialize_field("mutableContent", &enable)?;

        match self {
            IosRichTextType::None => Ok(()),
            IosRichTextType::Picture(image_url) => {
                struct_serialize.serialize_field("attachmentType", &1)?;
                struct_serialize.serialize_field("attachment", image_url)
            }
            IosRichTextType::Video(video_url) => {
                struct_serialize.serialize_field("attachmentType", &2)?;
                struct_serialize.serialize_field("attachment", video_url)
            }
            IosRichTextType::Voice(voice_url) => {
                struct_serialize.serialize_field("attachmentType", &3)?;
                struct_serialize.serialize_field("attachment", voice_url)
            }
        }
    }
}
