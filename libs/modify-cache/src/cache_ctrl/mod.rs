use std::{borrow::Cow, ops::Add};

use axum_resp_result::{ExtraFlag, ExtraFlags};
use http::{
    header::{HeaderName, CONTENT_LOCATION, VARY},
    Uri,
};

use self::control::CacheControl;

pub mod control;

#[derive(Debug, Default)]
pub struct CacheHeaders {
    pub(crate) content_local: Option<Uri>,
    pub(crate) vary_headers: Vec<HeaderName>,
    pub(crate) control: CacheControl,
}

impl CacheHeaders {
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

impl<'c> Add<&'c CacheHeaders> for ExtraFlags {
    type Output = ExtraFlags;

    fn add(mut self, rhs: &'c CacheHeaders) -> Self::Output {
        if let Some(uri) = &rhs.content_local {
            self +=
                ExtraFlag::insert_header(CONTENT_LOCATION, uri.to_string());
        }
        if let Some(s) = rhs
            .vary_headers
            .iter()
            .map(|h| h.as_str())
            .map(Cow::Borrowed)
            .reduce(|l, r| format!("{l}, {r}").into())
        {
            self += ExtraFlag::insert_header(VARY, s.to_string())
        }

        self + &rhs.control
    }
}

impl<'c> std::ops::AddAssign<&'c CacheHeaders> for ExtraFlags {
    fn add_assign(&mut self, rhs: &'c CacheHeaders) {
        let this = std::mem::replace(self, ExtraFlags::from(()));
        let _ = std::mem::replace(self, this + rhs);
    }
}

#[cfg(test)]
mod test {
    use http::header::{AUTHORIZATION, IF_MODIFIED_SINCE, IF_NONE_MATCH};

    use super::CacheHeaders;

    #[test]
    fn test_extra_headers() {
        let mut d = CacheHeaders::default();
        d.add_vary_headers([IF_NONE_MATCH, IF_MODIFIED_SINCE, AUTHORIZATION]);

        println!("{d:?}")
    }
}
