use http::{
    header::{InvalidHeaderValue, CONTENT_TYPE, HOST},
    HeaderName, HeaderValue,
};
use serde_json::Value::String;
use smallstr::SmallString;
use url::{Position, Url};

use crate::task_trait::{
    serde_content::SerializeContentTrait, task_content::TaskContent,
    task_request::TaskRequestTrait,
};

/// 获得特定请求头内容
pub trait HeaderFetch<T>
where
    T: TaskRequestTrait,
{
    /// 待获取的请求头的名称
    fn name(&self) -> HeaderName;

    /// 从请求任务、URL获得请求头信息
    fn fetch_header(
        &self, task: &T, url: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue>;
}

pub type DynFetch<'r, T> = &'r dyn HeaderFetch<T>;

pub struct ContentType;

impl<T> HeaderFetch<T> for ContentType
where
    T: TaskRequestTrait,
{
    fn name(&self) -> HeaderName { CONTENT_TYPE }

    fn fetch_header(
        &self, task: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        let ty = SerializeContentTrait::content_type(
            &<T as TaskContent>::payload(task),
        );
        HeaderValue::from_str(ty.as_ref())
    }
}

pub struct Host;

impl<T: TaskRequestTrait> HeaderFetch<T> for Host {
    fn name(&self) -> HeaderName { HOST }

    fn fetch_header(
        &self, _: &T, url: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&url[Position::BeforeHost..Position::AfterHost])
    }
}

pub struct TcAction;

impl<T: TaskRequestTrait> HeaderFetch<T> for TcAction {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-action") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(T::ACTION)
    }
}

pub struct FormattedRequiredHeaders {
    pub headers: SmallString<[u8; 32]>,
    pub formatted_headers: SmallString<[u8; 128]>,
}

pub fn get_required_headers<T: TaskRequestTrait>(
    headers: &[DynFetch< T>], task: &T, url: &Url,
) -> Result<FormattedRequiredHeaders, crate::error::TcCloudError> {
    let mut headers_iter = headers.iter().peekable();
    let mut headers = SmallString::new();
    let mut formatted_headers = SmallString::new();
    use core::fmt::Write;
    while let Some(fetcher) = headers_iter.next() {
        let name = fetcher.name();
        let value = fetcher
            .fetch_header(task, url)?
            .to_str()
            .unwrap()
            .to_owned();
        // last item
        if headers_iter.peek().is_none() {
            write!(&mut headers, "{name}")?;
            write!(&mut formatted_headers, "{name}:{value}")?;
        }
        else {
            writeln!(&mut headers, "{name}")?;
            writeln!(&mut formatted_headers, "{name}:{value}")?;
        }
    }
    Ok(FormattedRequiredHeaders {
        headers,
        formatted_headers,
    })
}
