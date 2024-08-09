use std::fmt::{Display, Formatter};
use serde::Deserialize;
use persistence::ceobe_operate::models::version::models::ReleasePlatform;

#[derive(Deserialize, Clone, Debug)]
pub struct QueryReleaseVersion {
    pub version: Option<semver::Version>,
    pub platform: ReleasePlatform,
}

impl Display for QueryReleaseVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            Some(ver) => write!(f, "{}:{}", self.platform, ver),
            None => write!(f, "{}", self.platform),
        }
    }
}
