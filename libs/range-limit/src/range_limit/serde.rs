use std::ops::Deref;

use serde::{Serialize, de};

use crate::{measurable::Measurable, RangeBoundLimit, RangeBound};

trait SerdeProc {
    type ToSerde: Measurable+serde::Serialize;
    fn into_serde(&self) -> &Self::ToSerde;
}

struct Normal<T>(T);

impl<T: Measurable+Serialize> SerdeProc for Normal<T> {
    type ToSerde = T;

    fn into_serde(&self) -> &Self::ToSerde {
        &self.0
    }
}



struct SmartPtr<P, T>(P)
where
    P: Deref<Target = T>;

impl<P, T> SerdeProc for SmartPtr<P, T>
where
    P: Deref<Target = T>,
    T: Measurable+Serialize,
{
    type ToSerde = T;

    fn into_serde(&self) -> &Self::ToSerde {
        self.0.deref()
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
