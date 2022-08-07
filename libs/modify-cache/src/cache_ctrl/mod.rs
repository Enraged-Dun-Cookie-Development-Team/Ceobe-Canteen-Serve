pub mod control;
use std::{borrow::Cow, ops::Add};

use http::{
    header::{HeaderName, CONTENT_LOCATION, VARY},
    Uri,
};
use resp_result::{ExtraFlag, ExtraFlags};

use self::control::CacheControl;
#[derive(Debug, Default)]
pub struct CacheInfo {
    pub(crate) content_local: Option<Uri>,
    pub(crate) vary_headers: Vec<HeaderName>,
    pub(crate) control: CacheControl,
}

impl CacheInfo {
    pub fn clean_content_local(&mut self) -> &mut Self {
        self.content_local = None;
        self
    }

    pub fn set_content_local(&mut self, uri: Uri) -> &mut Self {
        self.content_local.replace(uri);
        self
    }

    pub fn add_vary_headers<I: IntoIterator<Item = HeaderName>>(
        &mut self, headers: I,
    ) -> &mut Self {
        headers.into_iter().for_each(|header| {
            if !self.vary_headers.contains(&header) {
                self.vary_headers.push(header)
            }
        });
        self
    }

    pub fn get_control(&mut self) -> &mut CacheControl { &mut self.control }
}

impl<'c> Add<&'c CacheInfo> for ExtraFlags {
    type Output = ExtraFlags;

    fn add(mut self, rhs: &'c CacheInfo) -> Self::Output {
        if let Some(uri) = &rhs.content_local {
            self = self
                + ExtraFlag::insert_header(CONTENT_LOCATION, uri.to_string());
        }
        if let Some(s) = rhs
            .vary_headers
            .iter()
            .map(|h| h.as_str())
            .map(Cow::Borrowed)
            .reduce(|l, r| format!("{l}, {r}").into())
        {
            self = self + ExtraFlag::insert_header(VARY, s.to_string())
        }

        self + &rhs.control
    }
}

#[cfg(test)]
mod test {
    use http::header::{AUTHORIZATION, IF_MODIFIED_SINCE, IF_NONE_MATCH};

    use super::CacheInfo;

    #[test]
    fn test_extra_headers() {
        let mut d = CacheInfo::default();
        d.add_vary_headers([IF_NONE_MATCH, IF_MODIFIED_SINCE, AUTHORIZATION]);

        println!("{d:?}")
    }
}
