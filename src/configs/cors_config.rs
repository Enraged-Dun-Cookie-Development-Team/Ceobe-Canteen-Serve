use std::{collections::HashSet, sync::Arc};

use http::{HeaderValue, Method};
use serde::{Deserialize, Deserializer};

use crate::bootstrap::middleware::cors::CorsConfigTrait;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CorsConfigImpl {
    #[serde(alias = "origins", deserialize_with = "de_origins")]
    allow_origins: Vec<HeaderValue>,
    #[serde(alias = "methods", deserialize_with = "de_methods")]
    allow_methods: Vec<Method>,
    #[serde(alias = "paths")]
    bypass_paths: Arc<HashSet<String>>,
}

impl CorsConfigTrait for CorsConfigImpl {
    fn allow_origins(&self) -> Vec<HeaderValue> { self.allow_origins.clone() }

    fn allow_methods(&self) -> Vec<Method> { self.allow_methods.clone() }

    fn bypass_paths(&self) -> Arc<HashSet<String>> {
        Arc::clone(&self.bypass_paths)
    }
}

fn de_origins<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Vec<HeaderValue>, D::Error> {
    let vec = Vec::<String>::deserialize(de)?;
    vec.iter().map(|path| path.parse()).try_fold(
        Vec::with_capacity(vec.len()),
        |mut vec, value| {
            vec.push(value.map_err(serde::de::Error::custom)?);
            Ok(vec)
        },
    )
}

fn de_methods<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Vec<Method>, D::Error> {
    let vec = Vec::<String>::deserialize(de)?;
    vec.iter()
        .map(|method| Method::try_from(method.as_str()))
        .try_fold(Vec::with_capacity(vec.len()), |mut vec, value| {
            vec.push(value.map_err(serde::de::Error::custom)?);
            Ok(vec)
        })
}
