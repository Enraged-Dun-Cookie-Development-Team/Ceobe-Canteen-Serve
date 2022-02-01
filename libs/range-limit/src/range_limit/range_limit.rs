use std::{
    fmt::{Debug, Display},
    ops::Deref,
};

use serde::de;

use crate::{
    error::{self},
    measurable::Measurable,
};

use super::{RangeBound, SizeStatus};

pub struct RangeBoundLimit<T, Rb>(T, Rb);

impl<T, Rb: Default> RangeBoundLimit<T, Rb> {
    fn handle_arms(status: SizeStatus, size: usize, value: T) -> Result<Self, error::Error> {
        match status {
            SizeStatus::Ok => Ok(Self(value, Rb::default())),
            SizeStatus::TooLarge(require) => Err(error::Error::TooLarget { require, get: size }),
            SizeStatus::TooSmall(require) => Err(error::Error::TooSmall { require, get: size }),
            SizeStatus::FIxSize(s) => Err(error::Error::FixSize {
                require: s,
                get: size,
            }),
            SizeStatus::Costom(err) => Err(error::Error::Coutom(err)),
        }
    }
}

impl<T: Debug + Measurable, Rb: Debug> Debug for RangeBoundLimit<T, Rb> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeLimit")
            .field("data", &self.0)
            .field("bound", &self.1)
            .field("exact lenght", &self.0.size())
            .finish()
    }
}

impl<T: Display, Rb> Display for RangeBoundLimit<T, Rb> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T, Rb> Deref for RangeBoundLimit<T, Rb> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P, T, Rb> RangeBoundLimit<P, Rb>
where
    P: Deref<Target = T>,
    T: Measurable,
    Rb: RangeBound,
{
    pub fn try_from_ptr(value: P) -> Result<Self, error::Error> {
        Self::handle_arms(Rb::match_range(value.size()), value.size(), value)
    }
}

impl<T: Measurable, Rb: RangeBound> RangeBoundLimit<T, Rb> {
    pub fn try_from(value: T) -> Result<Self, error::Error> {
        Self::handle_arms(Rb::match_range(value.size()), value.size(), value)
    }

    pub fn into(self) -> T {
        self.0
    }
}

impl<T: Measurable, Rb> Measurable for RangeBoundLimit<T, Rb> {
    fn size(&self) -> usize {
        self.0.size()
    }
}

impl<T: serde::Serialize, Rb> serde::Serialize for RangeBoundLimit<T, Rb> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, Rb> serde::Deserialize<'de> for RangeBoundLimit<T, Rb>
where
    T: serde::Deserialize<'de> + Measurable,
    Rb: RangeBound,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tmp = T::deserialize(deserializer)?;
        Self::try_from(tmp).or_else(|e| Err(de::Error::custom(e)))
    }
}
