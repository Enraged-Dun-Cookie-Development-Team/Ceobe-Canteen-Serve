use std::fmt::{Display, Formatter};

use checker::SerdeCheck;
use page_size::request::PageSizeChecker;
use persistence::ceobe_operate::models::version::models::{
    DownloadSourceItem, ReleasePlatform,
};
use semver::Version;
use serde::Deserialize;
use serve_utils::{OptionField, OptionViewField, ValueField};

#[derive(Deserialize, Clone, Debug)]
pub struct QueryReleaseVersion<
    Version: OptionViewField<semver::Version> = OptionField<semver::Version>,
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

impl Display for QueryReleaseVersion<OptionField<Version>> {
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

#[derive(Debug, Deserialize)]
pub struct QueryVersionFilter {
    pub platform: Option<ReleasePlatform>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(flatten)]
    pub paginator: SerdeCheck<PageSizeChecker>,
}

impl Display for QueryVersionFilter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.platform {
            None => {
                write!(f, "{{deleted: {} }}", self.deleted)
            }
            Some(plat) => {
                write!(f, "{{deleted: {}, platform: {}}}", self.deleted, plat)
            }
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryVersionUpdate {
    #[serde(flatten)]
    pub version: QueryReleaseVersion<ValueField<Version>>,
    #[serde(flatten)]
    pub set: UpdatePayload,
}
#[derive(Debug, Deserialize, Default)]
pub struct UpdatePayload {
    pub description: Option<String>,
    pub download_source: Vec<DownloadSourceItem>,
}

#[cfg(test)]
mod test {

    use checker::SerdeCheck;
    use http::Uri;
    use page_size::request::PageSizeChecker;
    use serde::Deserialize;
    use serve_utils::{axum::extract::Query, SkipField};

    use super::QueryVersionFilter;
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

    #[test]
    fn test_query_query() {
        // QueryVersionFilter
        let uri: Uri =
            "http://example.com/path?deleted=false&page=11&size=12"
                .parse()
                .expect("Bad uri");
        let reg = Query::<QueryVersionFilter>::try_from_uri(&uri)
            .expect("Error Parse Query");

        println!("{reg:?}")
    }
}
