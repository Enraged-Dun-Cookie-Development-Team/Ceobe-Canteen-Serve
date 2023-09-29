use http::{HeaderValue, Method};
use serde::{Deserialize, Serialize};
use cors::CorsConfig;

#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct CorsConfigImpl {
    #[serde(alias = "origins")]
    allow_origins: Vec<String>,
    #[serde(alias = "methods")]
    allow_methods: Vec<Method>,
}

impl CorsConfig for CorsConfigImpl {
    fn allow_origins(&self) -> Vec<HeaderValue> {
        self.allow_origins.iter().map(|path| path.parse().expect("Bad Origin Value")).collect()
    }

    fn allow_methods(&self) -> &[Method] {
        &self.allow_methods
    }
}