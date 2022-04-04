use std::{convert::Infallible, marker::PhantomData, vec::IntoIter};

use futures::future::ok;
use futures_util::future::Ready;

pub enum HeaderInfo<H> {
    Exist(Vec<String>, PhantomData<H>),
    None(PhantomData<H>),
}

impl<H> IntoIterator for HeaderInfo<H>
where
    H: FromHeaders,
{
    type IntoIter = IntoIter<String>;
    type Item = String;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            HeaderInfo::Exist(v, _) => v.into_iter(),
            HeaderInfo::None(_) => vec![].into_iter(),
        }
    }
}

#[allow(dead_code)]
impl<H> HeaderInfo<H>
where
    H: FromHeaders,
{
    pub fn get_one(self) -> Option<String> {
        match self {
            HeaderInfo::Exist(v, _) => Some(v.into_iter().next().unwrap()),
            HeaderInfo::None(_) => None,
        }
    }

    pub fn iter(&self) -> Option<impl Iterator<Item = &str>> {
        match self {
            HeaderInfo::Exist(v, _) => Some(v.iter().map(|s| s.as_str())),
            HeaderInfo::None(_) => None,
        }
    }
}

pub trait FromHeaders {
    fn header_name() -> &'static str;
}

impl<H> request_pretreat::Treater for HeaderInfo<H>
where
    H: FromHeaders,
{
    type Err = Infallible;
    type Fut = Ready<Result<Self::Resp, Self::Err>>;
    type Resp = Self;

    fn proc(
        req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload,
    ) -> Self::Fut {
        let header = req.headers();
        let res = header
            .get_all(H::header_name())
            .into_iter()
            .filter_map(|v| v.to_str().ok())
            .filter_map(|s| urlencoding::decode(s).ok())
            .map(|s| s.into_owned())
            .collect::<Vec<_>>();

        let result = if res.len() == 0 {
            Self::None(Default::default())
        }
        else {
            Self::Exist(res, Default::default())
        };
        ok(result)
    }
}

#[macro_export]
/// 辅助生成 [FromHeaders](crate::utils::data_struct::header_info::
/// FromHeaders) ```rust
///                 //    |-------------新建的类型的可见度
///                 //    |    |--------新建的类型的名称
///                 //    |    |          |---- 捕获的头类型
///     header_captures!(pub Referer: "referer");
/// ```
macro_rules! header_captures {
    ($v:vis $i:ident : $hn:literal) => {
        #[derive(Default)]
        $v struct $i;
        impl $crate::utils::data_struct::header_info::FromHeaders for $i{
            fn header_name() -> &'static str{
                $hn
            }
        }
    };
}

header_captures!(pub Referer: "referer");
