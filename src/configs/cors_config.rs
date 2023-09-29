use cors::CorsConfig;
use http::{HeaderValue, Method};
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, Clone, Default)]
pub struct CorsConfigImpl {
    #[serde(alias = "origins")]
    allow_origins: Vec<String>,
    #[serde(alias = "methods", deserialize_with = "de_methods")]
    allow_methods: Vec<Method>,
}

impl CorsConfig for CorsConfigImpl {
    fn allow_origins(&self) -> Vec<HeaderValue> {
        self.allow_origins
            .iter()
            .map(|path| path.parse().expect("Bad Origin Value"))
            .collect()
    }

    fn allow_methods(&self) -> &[Method] { &self.allow_methods }
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
