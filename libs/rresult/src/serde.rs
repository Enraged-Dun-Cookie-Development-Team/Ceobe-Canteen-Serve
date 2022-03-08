use std::ops::Deref;

use serde::{ser::SerializeStruct, Serialize};
use status_err::StatusErr;

use crate::{r_result::RResult};

impl<T, E> Serialize for RResult<T, E>
where
    T: for<'a> IntoSerde<'a>,
    E: StatusErr,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut stur = serializer.serialize_struct("ResResult", 3)?;
        match self {
            RResult::Success(data) => {
                stur.serialize_field("data", &data.into_serde())?;
            }
            RResult::Error(msg) => {
                stur.serialize_field("emsg", &StatusErr::information(msg) )?;
            }
        };
        stur.end()
    }
}

pub struct Wrap<T>(pub T);

impl<T> Deref for Wrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait IntoSerde<'s> {
    type Target: Serialize;
    fn into_serde(&'s self) -> Self::Target;
}

impl<'s, T> IntoSerde<'s> for T
where
    T: Deref,
    <T as Deref>::Target: Serialize + 's,
{
    type Target = &'s <T as Deref>::Target;

    fn into_serde(&'s self) -> Self::Target {
        self.deref()
    }
}
