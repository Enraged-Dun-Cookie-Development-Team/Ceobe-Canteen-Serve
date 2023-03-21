use serde::ser::SerializeStruct;

use crate::push_notify::NotifySerialize;
#[derive(Debug, Clone)]

pub enum Badge {
    Set(u32),
    Add(u32),
}

impl Badge {
    ///角标数值取 `num` 值
    ///
    /// ## Notify
    /// - 透传消息不支持
    pub fn new_set(num: u32) -> Self {
        Self::Set(num)
    }

    ///角标数值为 `base` 当前值加1
    ///
    /// ## Notify
    /// - 透传消息不支持
    pub fn new_add(base: u32) -> Self {
        Self::Add(base)
    }
}

impl NotifySerialize for Badge {
    fn serialize_field(&self) -> usize {
        2
    }

    fn serialize<S: serde::Serializer>(
        &self,
        struct_serialize: &mut <S as serde::Serializer>::SerializeStruct,
    ) -> Result<(), <S as serde::Serializer>::Error> {
        let (ty, value) = match self {
            Badge::Set(value) => (1, value),
            Badge::Add(value) => (2, value),
        };

        struct_serialize.serialize_field("androidBadgeType", &ty)?;
        struct_serialize.serialize_field("androidBadge", value)?;

        Ok(())
    }
}
