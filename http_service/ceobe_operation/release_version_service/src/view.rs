use std::fmt::{Display, Formatter};

use checker::SerdeCheck;
use page_size::request::PageSizeChecker;
use persistence::{
    ceobe_operate::models::version::models::{
        DownloadSourceItem, ReleasePlatform,
    },
    mongodb::mongodb::bson::oid::ObjectId,
};
use semver::Version;
use serde::Deserialize;
use serve_utils::{
    const_field::ConstBoolField, OptionField, OptionViewField, ValueField,
};

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
pub struct QueryVersionFilter<
    Delete: OptionViewField<bool>,
    Platform: OptionViewField<ReleasePlatform> = OptionField<ReleasePlatform>,
> {
    pub platform: Platform,
    #[serde(default)]
    pub deleted: Delete,
    #[serde(flatten)]
    pub paginator: SerdeCheck<PageSizeChecker>,
}

impl<D: Display + OptionViewField<bool>> Display for QueryVersionFilter<D> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.platform.0 {
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
pub struct QueryVersionNextIdFilter<
    Delete: OptionViewField<bool> = ConstBoolField<false>,
    Platform: OptionViewField<ReleasePlatform> = ValueField<ReleasePlatform>,
> {
    pub platform: Platform,
    #[serde(default)]
    pub deleted: Delete,
    #[serde(default)]
    pub first_id: Option<ObjectId>,
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

    use http::Uri;
    use serve_utils::{axum::extract::Query, SkipField, ValueField};

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
        let reg =
            Query::<QueryVersionFilter<ValueField<bool>>>::try_from_uri(&uri)
                .expect("Error Parse Query");

        println!("{reg:?}")
    }
}
