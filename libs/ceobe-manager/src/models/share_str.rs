use std::{borrow::Borrow, ops::Deref, sync::Arc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct AShareString(Arc<String>);

impl Deref for AShareString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Borrow<String> for AShareString {
    fn borrow(&self) -> &String {
        self.0.deref()
    }
}

impl Borrow<str> for AShareString {
    fn borrow(&self) -> &str {
        self.0.deref()
    }
}

impl Clone for AShareString {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl Serialize for AShareString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AShareString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let resp = String::deserialize(deserializer)?;

        Ok(Self(Arc::new(resp)))
    }
}
