use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use serde::de;

use crate::{error, measurable::Measurable};

pub struct MinLimit<T, const MIN: usize>(T);

impl<P, T, const MIN: usize> MinLimit<P, MIN>
where
    P: Deref<Target = T>,
    T: Measurable,
{
    pub fn try_from_ptr(value: P) -> Result<Self, error::Error> {
        if value.deref().size() > MIN {
            Ok(Self(value))
        } else {
            Err(error::Error::TooSmall {
                require: MIN,
                get: value.size(),
            })
        }
    }
}

impl<T: Display, const MIN: usize> Display for MinLimit<T, MIN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Debug + Measurable, const MIN: usize> Debug for MinLimit<T, MIN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinLimit")
            .field("data", &self.0)
            .field("min limit", &MIN)
            .field("exact lenght", &self.0.size())
            .finish()
    }
}

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
