use std::ops::Deref;

use serde::de;

use crate::{error, measurable::Measurable};

pub struct MinLimit<T, const MIN: usize>(T);

impl<T, const M: usize> Deref for MinLimit<T, M> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Measurable, const M: usize> MinLimit<T, M> {
    pub fn try_from(value: T) -> Result<Self, error::Error> {
        if value.size() > M {
            Ok(Self(value))
        } else {
            Err(error::Error::TooSmall {
                require: M,
                get: value.size(),
            })
        }
    }

    pub fn into(self) -> T {
        self.0
    }
}

impl<T: Measurable, const M: usize> Measurable for MinLimit<T, M> {
    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<T: serde::Serialize, const M: usize> serde::Serialize for MinLimit<T, M> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.deref().serialize(serializer)
    }
}

impl<'de, T: serde::Deserialize<'de> + Measurable, const M: usize> serde::Deserialize<'de>
    for MinLimit<T, M>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tmp = T::deserialize(deserializer)?;
        Self::try_from(tmp).or_else(|e| Err(de::Error::custom(e)))
    }
}
