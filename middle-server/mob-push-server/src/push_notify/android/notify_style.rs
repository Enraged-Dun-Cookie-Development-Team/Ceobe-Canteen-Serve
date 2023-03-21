use serde::{ser::SerializeStruct, Serialize};
use typed_builder::TypedBuilder;

use crate::push_notify::NotifySerialize;

#[derive(Debug, Clone)]
pub enum NotifyStyle {
    Default,
    LongContent(String),
    BigVision(String),
    Banner(Vec<String>),
    Custom(CustomStyle),
}

impl NotifyStyle {
    /// 创建一个 **默认样式**  的推送消息格式
    pub fn new_default() -> Self {
        Self::Default
    }
    /// 创建一个 **长文本** 的推送消息格式  
    ///
    /// 该推送模式将会使原有Content隐藏  
    pub fn new_long_content(content: impl Into<String>) -> Self {
        Self::LongContent(content.into())
    }

    /// 创建一个 **大图** 的推送消息格式
    ///  
    /// 该推送方法会同时保留图片和content  
    pub fn new_big_vision(image_url: impl Into<String>) -> Self {
        Self::BigVision(image_url.into())
    }

    /// 创建一个 **多行横幅** 的推送消息格式  
    ///
    /// 迭代器中每一元素一行
    ///   
    /// 该推送模式将会使原有Content隐藏  
    pub fn new_banner<I, T>(contents: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self::Banner(contents.into_iter().map(Into::into).collect())
    }
    /// 创建一个 **用户定义** 的推送消息格式  
    pub fn new_custom(custom_style: CustomStyle) -> Self {
        Self::Custom(custom_style)
    }
}

impl NotifyStyle {
    fn get_code(&self) -> i32 {
        match self {
            NotifyStyle::Default => 0,
            NotifyStyle::LongContent(_) => 1,
            NotifyStyle::BigVision(_) => 2,
            NotifyStyle::Banner(_) => 3,
            NotifyStyle::Custom(_) => 4,
        }
    }
}

#[derive(Debug, Clone)]
pub enum StyleId {
    // 1 样式1
    One,
    // 2 样式2
    Tow,
    // 3 样式3
    Three,
}

impl Serialize for StyleId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(self.to_code())
    }
}

impl StyleId {
    fn to_code(&self) -> i32 {
        match self {
            StyleId::One => 1,
            StyleId::Tow => 2,
            StyleId::Three => 3,
        }
    }
}

#[derive(Debug, Serialize, TypedBuilder, Clone)]
#[builder(field_defaults(default, setter(strip_option)))]
pub struct CustomStyle {
    /// 样式序号
    #[serde(rename = "styleNo", skip_serializing_if = "Option::is_none")]
    style: Option<StyleId>,

    /// 背景图Url
    #[serde(rename = "backgroundUrl", skip_serializing_if = "Option::is_none")]
    background_url: Option<String>,

    ///小图标
    #[serde(rename = "smallIcons", skip_serializing_if = "Option::is_none")]
    small_icons: Option<String>,

    ///按钮文案
    #[serde(rename = "buttonCopy", skip_serializing_if = "Option::is_none")]
    button_copy: Option<String>,

    ///点击按钮跳转的链接
    #[serde(rename = "buttonJumpUrl", skip_serializing_if = "Option::is_none")]
    button_jump_url: Option<String>,
}

impl NotifySerialize for NotifyStyle {
    fn serialize_field(&self) -> usize {
        match self {
            NotifyStyle::Default => 1,
            NotifyStyle::LongContent(_)
            | NotifyStyle::BigVision(_)
            | NotifyStyle::Banner(_)
            | NotifyStyle::Custom(_) => 2,
        }
    }

    fn serialize<S: serde::Serializer>(
        &self,
        serialize_struct: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        serialize_struct.serialize_field("style", &self.get_code())?;
        match self {
            NotifyStyle::Default => Ok(()),
            NotifyStyle::LongContent(s) | NotifyStyle::BigVision(s) => {
                serialize_struct.serialize_field("content", &[s])
            }
            NotifyStyle::Banner(info_vec) => serialize_struct.serialize_field("content", info_vec),
            NotifyStyle::Custom(inner) => serialize_struct.serialize_field("customStyle", inner),
        }
    }
}
