use std::fmt::{Display, Formatter};

use persistence::ceobe_operate::models::version::models::{
    DownloadSourceItem, ReleasePlatform,
};
use semver::Version;
use serde::Deserialize;
use serve_utils::{OptionValueField, OptionViewField, ValueField};

#[derive(Deserialize, Clone, Debug)]
pub struct QueryReleaseVersion<
    Version: OptionViewField<semver::Version> = OptionValueField<
        semver::Version,
    >,
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

#[derive(Debug, Deserialize, Default)]
pub struct QueryVersionFilter {
    pub platform: Option<ReleasePlatform>,
    pub yanked: bool,
}

impl Display for QueryVersionFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.platform {
            None => {
                write!(f, "{{yanked: {} }}", self.yanked)
            }
            Some(plat) => {
                write!(f, "{{yanked: {}, platform: {}}}", self.yanked, plat)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryVersionUpdate {
    pub version: QueryReleaseVersion<ValueField<Version>>,
    #[serde(rename = "$set")]
    pub set: UpdatePayload,
}
#[derive(Debug, Deserialize, Default)]
pub struct UpdatePayload {
    pub description: Option<String>,
    pub download_source: Vec<DownloadSourceItem>,
}

#[cfg(test)]
mod test {
    use serde::Deserialize;
    use serde_json::json;
    use serve_utils::SkipField;

    use crate::view::QueryReleaseVersion;

    #[test]
    fn test_de() {
        let js = serde_json::json!({
            "platform":"desktop"
        });
        let v = serde_json::from_value::<QueryReleaseVersion<SkipField>>(js)
            .expect("Err");

        println!("{v:?}")
    }
}
