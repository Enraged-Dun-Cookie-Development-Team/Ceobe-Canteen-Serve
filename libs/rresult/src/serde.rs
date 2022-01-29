use std::ops::Deref;

use serde::{ser::SerializeStruct, Serialize};

use crate::r_result::RResult;

impl<T, I, E> Serialize for RResult<T, E>
where
    T: Deref<Target = I>,
    I: Serialize,
    E: std::error::Error,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut stur = serializer.serialize_struct("ResResult", 3)?;
        match self {
            RResult::Success(data) => {
                stur.serialize_field("err", &false)?;
                stur.serialize_field("emsg", "")?;
                stur.serialize_field("data", &data.deref())?;
            }
            RResult::Error(_status, msg) => {
                stur.serialize_field("err", &true)?;
                stur.serialize_field("emsg", &format!("Error: {}", msg))?;
                stur.serialize_field("data", &Option::<I>::None)?;
            }
        };
        stur.end()
    }
}

pub struct Wrap<T> (pub T);

impl<T> Deref for Wrap<T> {
    type Target=T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

