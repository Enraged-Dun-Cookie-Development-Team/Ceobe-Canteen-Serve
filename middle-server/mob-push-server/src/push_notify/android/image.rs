use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]

pub enum Image {
    Icon(String),
    Image(String),
}

impl Image {
    /// 附带小图标的推送
    ///
    /// ## Notify
    ///
    /// 目前客户端版本暂不支持
    pub fn new_icon(icon_url: impl Into<String>) -> Self {
        Self::Icon(icon_url.into())
    }

    ///推送大图标的url地址
    ///
    /// ## Notify
    ///
    /// - 透传消息不支持
    /// - 小米厂商对图片尺寸有严格要求，不符合要求则不会按照大图样式进行推送，
    /// 具体要求为：宽高固定为876*324px，格式需为PNG/JPG/JPEG，大小小于1M
    /// - OPPO厂商大图需要申请权限，否则会报错导致客户端收不到推送消息
    pub fn new_image(image_url: impl Into<String>) -> Self {
        Self::Image(image_url.into())
    }
}

impl NotifySerialize for Image {
    fn serialize_field(&self) -> usize {
        1
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        match self {
            Image::Icon(url) => struct_serialize.serialize_field("icon", url),
            Image::Image(url) => struct_serialize.serialize_field("image", url),
        }
    }
}
