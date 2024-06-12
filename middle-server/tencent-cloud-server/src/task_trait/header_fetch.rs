use http::{
    header::{InvalidHeaderValue, CONTENT_TYPE, HOST},
    HeaderName, HeaderValue,
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
            .to_owned();
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

#[cfg(test)]
mod test {
    use url::Url;

    use crate::{
        cloud_manager::entities::Service,
        task_trait::{
            header_fetch::{
                get_required_headers, ContentType, Host, TcAction,
            },
            serde_content::Json,
            task_content::TaskContent,
            task_request::TaskRequestTrait,
        },
    };
    use crate::cloud_manager::entities::ServerVersion;

    #[test]
    fn test_head_gen() {
        struct Test;
        impl TaskContent for Test {
            type Payload<'r> = Json<'r, str>;

            fn payload(&self) -> Self::Payload<'_> { Json("Acv") }
        }
        impl TaskRequestTrait for Test {
            const ACTION: &'static str = "Action";
            const VERSION: ServerVersion = ServerVersion::Ver20180606;
            const SERVICE: Service = Service::Cdn;
        }

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
}
