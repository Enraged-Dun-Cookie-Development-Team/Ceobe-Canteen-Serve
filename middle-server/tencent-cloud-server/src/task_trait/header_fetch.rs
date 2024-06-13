use chrono::{DateTime, Utc};
use http::{
    header::{InvalidHeaderValue, AUTHORIZATION, CONTENT_TYPE, HOST},
    HeaderMap, HeaderName, HeaderValue,
};
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
        HeaderValue::from_str(&ty.as_ref().to_lowercase())
    }
}

pub struct Host;

impl<T: TaskRequestTrait> HeaderFetch<T> for Host {
    fn name(&self) -> HeaderName { HOST }

    fn fetch_header(
        &self, _: &T, url: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(
            &url[Position::BeforeHost..Position::AfterHost].to_lowercase(),
        )
    }
}

pub struct TcAction;

impl<T: TaskRequestTrait> HeaderFetch<T> for TcAction {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-action") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&T::ACTION.to_lowercase())
    }
}

pub struct TcVersion;

impl<T: TaskRequestTrait> HeaderFetch<T> for TcVersion {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-version") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        Ok(T::VERSION.header_value())
    }
}

pub struct TcTimestamp<'t>(pub &'t DateTime<Utc>);

impl<'t, T: TaskRequestTrait> HeaderFetch<T> for TcTimestamp<'t> {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-timestamp") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(&self.0.timestamp().to_string())
    }
}

pub struct Authorization<'auth>(pub &'auth str);

impl<'auth, T: TaskRequestTrait> HeaderFetch<T> for Authorization<'auth> {
    fn name(&self) -> HeaderName { AUTHORIZATION }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(self.0)
    }
}

pub struct TcRegion;

impl<T: TaskRequestTrait> HeaderFetch<T> for TcRegion {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-region") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(T::REGION.unwrap_or_default())
    }
}

pub struct TcToken;

impl<T: TaskRequestTrait> HeaderFetch<T> for TcToken {
    fn name(&self) -> HeaderName { HeaderName::from_static("x-tc-token") }

    fn fetch_header(
        &self, _: &T, _: &Url,
    ) -> Result<HeaderValue, InvalidHeaderValue> {
        HeaderValue::from_str(T::TOKEN.unwrap_or_default())
    }
}

pub struct FormattedRequiredHeaders {
    pub headers: SmallString<[u8; 32]>,
    pub formatted_headers: SmallString<[u8; 128]>,
}

pub fn get_required_headers<T: TaskRequestTrait>(
    headers: &[DynFetch<T>], task: &T, url: &Url,
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
            .to_lowercase();
        // last item
        let sep = if headers_iter.peek().is_none() {
            ""
        }
        else {
            ";"
        };
        write!(&mut headers, "{name}{sep}")?;
        writeln!(&mut formatted_headers, "{name}:{value}")?;
    }

    Ok(FormattedRequiredHeaders {
        headers,
        formatted_headers,
    })
}
pub fn to_header_map<'i, T: TaskRequestTrait + 'i>(
    headers: impl IntoIterator<Item = &'i dyn HeaderFetch<T>>, task: &T,
    url: &Url,
) -> Result<HeaderMap, crate::error::TcCloudError> {
    let mut header_map = HeaderMap::new();

    for header in headers {
        header_map.append(header.name(), header.fetch_header(task, url)?);
    }
    Ok(header_map)
}

#[cfg(test)]
mod test {
    use chrono::Utc;
    use url::Url;

    use crate::{
        cloud_manager::entities::{ServerVersion, Service},
        task_trait::{
            header_fetch::{
                get_required_headers, to_header_map, Authorization,
                ContentType, HeaderFetch, Host, TcAction, TcRegion,
                TcTimestamp, TcVersion,
            },
            serde_content::Json,
            task_content::TaskContent,
            task_request::TaskRequestTrait,
        },
    };
    use crate::task_trait::header_fetch::TcToken;

    struct Test;
    impl TaskContent for Test {
        type Payload<'r> = Json<'r, str>;

        fn payload(&self) -> Self::Payload<'_> { Json("Acv") }
    }
    impl TaskRequestTrait for Test {
        const ACTION: &'static str = "Action";
        const SERVICE: Service = Service::Cdn;
        const VERSION: ServerVersion = ServerVersion::Ver20180606;

        const TOKEN: Option<&'static str> = Some("Test_token");

        const REGION: Option<&'static str> = Some("Test-Regin");
    }
    #[test]
    fn test_head_gen() {
        let url = Url::parse("http://www.example.com/abc").unwrap();

        let ret = get_required_headers(
            &[&ContentType, &Host, &TcAction],
            &Test,
            &url,
        )
        .expect("error");

        assert_eq!(ret.headers, "content-type;host;x-tc-action");
        assert_eq!(
            ret.formatted_headers,
            "content-type:application/json; \
             charset=utf-8\nhost:www.example.com\nx-tc-action:action\n"
        )
    }
    #[test]
    fn test_header_map() {
        let url = Url::parse("http://www.example.com/abc").unwrap();

        let ret = to_header_map(
            [
                &Host as &dyn HeaderFetch<Test>,
                &TcAction,
                &TcVersion,
                &TcTimestamp(&Utc::now()),
                &ContentType,
                &Authorization("ABC"),
            ]
            .into_iter()
            .chain(Test::REGION.map(|_| &TcRegion as _))
            .chain(Test::TOKEN.map(|_| &TcToken as _)),
            &Test,
            &url,
        )
        .expect("Error");
        
        println!("{ret:?}")
    }
}
