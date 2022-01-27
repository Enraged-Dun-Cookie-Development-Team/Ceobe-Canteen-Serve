use std::vec::IntoIter;

use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;

pub enum HeaderInfo<'s, H> {
    Exist(Vec<&'s str>, H),
    None(H),
}

impl<'s, H> IntoIterator for HeaderInfo<'s, H>
where
    H: FromHeaders,
{
    type Item = &'s str;

    type IntoIter = IntoIter<&'s str>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            HeaderInfo::Exist(v, _) => v.into_iter(),
            HeaderInfo::None(_) => vec![].into_iter(),
        }
    }
}

impl<'s, H> HeaderInfo<'s, H>
where
    H: FromHeaders,
{
    pub fn get_one(self) -> Option<&'s str> {
        match self {
            HeaderInfo::Exist(v, _) => Some(unsafe { v.get_unchecked(0) }),
            HeaderInfo::None(_) => None,
        }
    }

}

pub trait FromHeaders {
    fn header_name() -> &'static str;
}

#[rocket::async_trait]
impl<'r, H> FromRequest<'r> for HeaderInfo<'r, H>
where
    H: FromHeaders + Default,
{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let sign = H::default();
        let headers = request.headers();
        let res = headers.get(H::header_name()).collect::<Vec<_>>();
        let result = if res.len() == 0 {
            Self::None(sign)
        } else {
            Self::Exist(res, sign)
        };
        Outcome::Success(result)
    }
}

#[macro_export]
macro_rules! header_captures {
    ($v:vis $i:ident : $hn:literal) => {
        #[derive(Default)]
        $v struct $i;
        impl $crate::utils::data_structs::header_info::FromHeaders for $i{
            fn header_name() -> &'static str{
                $hn
            }
        }
    };
}
