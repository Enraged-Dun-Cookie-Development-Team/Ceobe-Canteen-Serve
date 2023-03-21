use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]
/// 角标
pub enum IosBadgeType {
    /// 绝对值，不能为负数
    Abs(u32),
    /// 角标增减，(正数增加，负数减少)，减到0以下会设置为0
    Adding(i32),
}

impl NotifySerialize for IosBadgeType {
    fn serialize_field(&self) -> usize {
        2
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        match self {
            IosBadgeType::Abs(value) => {
                struct_serialize.serialize_field("badge", value)?;
                struct_serialize.serialize_field("badgeType", &1)
            }
            IosBadgeType::Adding(value) => {
                struct_serialize.serialize_field("badge", value)?;
                struct_serialize.serialize_field("badgeType", &2)
            }
        }
    }
}
