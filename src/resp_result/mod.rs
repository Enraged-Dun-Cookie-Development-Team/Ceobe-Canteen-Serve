
use crate::resp_error::RespError;

mod serde;
mod to_response;
mod try_op;

pub enum RespResult<T, E> {
    Success(T),
    Err(E),
}

impl<T, E> RespResult<T, E> {
    #[inline]
    pub fn map<N, F>(self, f: F) -> RespResult<N, E>
    where
        F: FnOnce(T) -> N,
    {
        #[cfg(feature = "log")]
        logger::info!(
            "RespResult Mapping Success From `{}` to `{}`",
            std::any::type_name::<T>(),
            std::any::type_name::<N>()
        );
        match self {
            RespResult::Success(data) => RespResult::Success(f(data)),
            RespResult::Err(e) => RespResult::Err(e),
        }
    }

    #[inline]
    pub fn map_err<N, F>(self, f: F) -> RespResult<T, N>
    where
        F: FnOnce(E) -> N,
    {
        #[cfg(feature = "log")]
        logger::info!(
            "RespResult Mapping Error From `{}` to `{}`",
            std::any::type_name::<E>(),
            std::any::type_name::<N>()
        );
        match self {
            RespResult::Success(data) => RespResult::Success(data),
            RespResult::Err(e) => RespResult::Err(f(e)),
        }
    }

    #[inline]
    pub fn and_then<N, F>(self, f: F) -> RespResult<N, E>
    where
        F: FnOnce(T) -> RespResult<N,E>,
    {
        match self {
            RespResult::Success(data) => f(data),
            RespResult::Err(e) => RespResult::Err(e),
        }
    }

    #[inline]
    pub fn or_else<N, F>(self, f: F) -> RespResult<T, N>
    where
        F: FnOnce(E) -> RespResult<T,N>,
    {
        match self {
            RespResult::Success(data) => RespResult::Success(data),
            RespResult::Err(e) => f(e),
        }
    }
}

impl<T, E> From<Result<T, E>> for RespResult<T, E>
where
    E: RespError,
{
    #[inline]
    fn from(r: Result<T, E>) -> Self {
        match r {
            Ok(data) => Self::ok(data),
            Err(err) => Self::err(err),
        }
    }
}

impl<T, E> RespResult<T, E> {
    #[inline]
    pub fn ok(data: T) -> Self {
        #[cfg(feature = "log")]
        logger::info!("RespResult 成功分支",);
        Self::Success(data)
    }
    #[inline]
    pub fn err(err: E) -> Self
    where
        E: RespError,
    {
        #[cfg(feature = "log")]
        logger::error!(
            "RespResult 异常分支 {} => {}",
            std::any::type_name::<E>(),
            err.description()
        );
        Self::Err(err)
    }
}
