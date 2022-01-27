use rocket::form::FromFormField;
use serde::{de, Deserialize, Serialize};
use std::{
    borrow::Borrow,
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
};

use super::measureable::Measurable;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct RangeLimit<T, const L: usize, const H: usize>
where
    T: Measurable,
{
    data: T,
}

impl<T: Measurable, const L: usize, const H: usize> RangeLimit<T, L, H> {
    pub fn try_from(value: T) -> Result<Self, LimitError> {
        if L <= value.size() && value.size() <= H {
            Ok(Self { data: value })
        } else {
            Err(LimitError::new(L, H, value.size()))
        }
    }
    pub fn into(self) -> T {
        self.data
    }
}

impl<T, const L: usize, const H: usize> Serialize for RangeLimit<T, L, H>
where
    T: Measurable + Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, T, const L: usize, const H: usize> Deserialize<'de> for RangeLimit<T, L, H>
where
    T: Measurable + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let temp = T::deserialize(deserializer)?;
        match Self::try_from(temp) {
            Err(msg) => Err(de::Error::custom(msg)),
            Ok(res) => Ok(res),
        }
    }
}

impl<T: Measurable, const L: usize, const H: usize> AsRef<T> for RangeLimit<T, L, H> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: Measurable, const L: usize, const H: usize> Deref for RangeLimit<T, L, H> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Measurable, const L: usize, const H: usize> Borrow<T> for RangeLimit<T, L, H> {
    fn borrow(&self) -> &T {
        &self.data
    }
}

impl<T: Measurable + Debug, const L: usize, const H: usize> Debug for RangeLimit<T, L, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RangeLimit")
            .field("data", &self.data)
            .field("min_size", &L)
            .field("max_size", &H)
            .field("size", &self.data.size())
            .finish()
    }
}

impl<T: Measurable + Display, const L: usize, const H: usize> Display for RangeLimit<T, L, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}

impl<'v, const L: usize, const H: usize> FromFormField<'v> for RangeLimit<String, L, H> {
    fn from_value(field: rocket::form::ValueField<'v>) -> rocket::form::Result<'v, Self> {
        let s = String::from_value(field)?;
        let res = Self::try_from(s).or_else(|e| Err(rocket::form::Error::custom(e)))?;
        Ok(res)
    }
}


#[derive(Debug)]
pub struct LimitError {
    min: usize,
    max: usize,
    real: usize,
}

impl LimitError {
    fn new(min: usize, max: usize, real: usize) -> Self {
        Self { min, max, real }
    }
}

impl Error for LimitError {}

impl Display for LimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Expect range between {} and {} ,but get {}",
            self.min, self.max, self.real
        )
    }
}
