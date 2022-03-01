use crate::RResult;

pub trait IntoRResult<T, E> {
    fn into_result(self) -> RResult<T, E>;
}

pub trait IntoRResultWithCodeError<T, E> {
    fn into_result_status(self, status: E) -> RResult<T, E>;
}

impl<T, E> IntoRResult<T, E> for Result<T, E> {
    fn into_result(self) -> RResult<T, E> {
        match self {
            Ok(data) => RResult::ok(data),
            Err(e) => RResult::err(e),
        }
    }
}

impl<T, E> IntoRResultWithCodeError<T, E> for Option<T> {
    fn into_result_status(self, status: E) -> RResult<T, E> {
        match self {
            Some(data) => RResult::ok(data),
            None => RResult::err(status),
        }
    }
}

impl<T, E> IntoRResult<T, E> for RResult<T, E> {
    fn into_result(self) -> RResult<T, E> {
        self
    }
}



