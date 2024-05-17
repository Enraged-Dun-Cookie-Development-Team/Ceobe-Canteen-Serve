
use serde::{de, Deserialize, Serialize};

use crate::{measurable::Measurable, RangeBound, RangeBoundLimit};


impl<T: serde::Serialize, Rb> Serialize for RangeBoundLimit<T, Rb> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T, Rb> Deserialize<'de> for RangeBoundLimit<T, Rb>
where
    T: Deserialize<'de> + Measurable,
    Rb: RangeBound,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tmp = T::deserialize(deserializer)?;
        Self::try_from(tmp).map_err(de::Error::custom)
    }
}
