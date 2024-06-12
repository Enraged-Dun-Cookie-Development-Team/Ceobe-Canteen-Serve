use chrono::{NaiveDate, Utc};
use http::Method;
use url::Url;
use general_request_client::traits::Requester;

use crate::{
    cloud_manager::entities::{ServerVersion, Service},
    task_trait::{
        header_fetch::{ContentType, DynFetch, Host, TcAction},
        task_content::TaskContent,
    },
};

pub trait TaskRequestTrait: TaskContent + Sized {
    /// 这个请求的请求方法，默认为Post
    const METHOD: Method = Method::POST;
    /// 请求行为
    const ACTION: &'static str;
    /// 请求服务的版本
    const VERSION: ServerVersion = ServerVersion::Ver20180606;
    /// 请求服务
    const SERVICE: Service;
    /// 请求地区
    const REGION: Option<&'static str> = None;
    /// 请求 Token
    const TOKEN: Option<&'static str> = None;

    const ALGORITHM:&'static str = "TC3-HMAC-SHA256";
    /// 请求时的时间戳
    fn timestamp(&self) -> i64 { Utc::now().timestamp() }
    /// 请求时的日期
    fn date(&self) -> NaiveDate { Utc::now().date_naive() }

    /// 请求时的签名头
    fn required_sign_header(&self) -> &[DynFetch<'_, Self>] {
        &[&ContentType, &Host, &TcAction]
    }
}

