use std::fmt::{Display, Formatter};

use persistence::ceobe_operate::models::version::models::ReleasePlatform;
use semver::Version;
use serde::Deserialize;
use serve_utils::{OptionValueField, OptionViewField, ValueField};

#[derive(Deserialize, Clone, Debug)]
pub struct QueryReleaseVersion<
    Version: OptionViewField = OptionValueField<semver::Version>,
> {
    #[serde(skip_serializing_if = "OptionViewField::skip_serde")]
    pub version: Version,
    pub platform: ReleasePlatform,
}

impl Display for QueryReleaseVersion<ValueField<Version>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.platform, self.version.0)
    }
}

impl Display for QueryReleaseVersion<OptionValueField<Version>> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.version.0 {
            None => {
                write!(f, "{}", self.platform)
            }
            Some(ver) => {
                write!(f, "{}->{}", self.platform, ver)
            }
        }
    }
}
