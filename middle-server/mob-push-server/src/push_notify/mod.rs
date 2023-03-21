pub mod android;
pub mod ios;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use std::ops::{Deref, DerefMut};

pub trait NotifySerialize {
    fn serialize_field(&self) -> usize;
    fn serialize<S: Serializer>(
        &self,
        struct_serialize: &mut <S as Serializer>::SerializeStruct,
    ) -> Result<(), <S as Serializer>::Error>;
}

impl<T: NotifySerialize> NotifySerialize for Option<T> {
    fn serialize_field(&self) -> usize {
        match self {
            Some(inner) => NotifySerialize::serialize_field(inner),
            None => 0,
        }
    }

    fn serialize<S: Serializer>(
        &self,
        struct_serialize: &mut <S as Serializer>::SerializeStruct,
    ) -> Result<(), <S as Serializer>::Error> {
        match self {
            Some(inner) => NotifySerialize::serialize::<S>(inner, struct_serialize),
            None => Ok(()),
        }
    }
}

pub trait SerializeInformation: NotifySerialize + Sized {
    fn serialize_name() -> &'static str;

    fn need_serialize(&self) -> bool {
        NotifySerialize::serialize_field(self) > 0
    }

    fn into_notify(self) -> Notify<Self> {
        Notify(self)
    }
}

#[derive(Debug, Clone)]
pub struct Notify<N>(N);

impl<N> DerefMut for Notify<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<N> Deref for Notify<N> {
    type Target = N;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<N> Notify<N> {
    pub fn new(notify: N) -> Self {
        Self(notify)
    }
}

impl<T: NotifySerialize + SerializeInformation> From<T> for Notify<T> {
    fn from(inner: T) -> Self {
        Self(inner)
    }
}

impl<N> Serialize for Notify<N>
where
    N: SerializeInformation + NotifySerialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut serialize_struct =
            serializer.serialize_struct(N::serialize_name(), self.0.serialize_field())?;

        self.0.serialize::<S>(&mut serialize_struct)?;

        serialize_struct.end()
    }
}
