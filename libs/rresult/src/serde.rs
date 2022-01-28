use serde::{Serialize, ser::SerializeStruct};

use crate::r_result::RResult;

impl<T, E> Serialize for RResult<T, E>
where
    T: Serialize,
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
                stur.serialize_field("data", &data)?;
            }
            RResult::Error(_status, msg) => {
                stur.serialize_field("err", &true)?;
                stur.serialize_field("emsg", &format!("Error: {}", msg))?;
                stur.serialize_field("data", &Option::<T>::None)?;
            }
        };
        stur.end()
    }
}