use rocket_::http::Status;

use crate::Wrap;

#[derive(Debug, PartialEq, Eq)]
pub enum RResult<T, E = String> {
    Success(T),
    Error(Status, E),
}

impl<T, E> RResult<T, E> {
    fn new_success(data: T) -> Self {
        Self::Success(data)
    }

    fn new_status_err(status: Status, err: E) -> Self {
        Self::Error(status, err)
    }

    fn new_err(err: E) -> Self {
        Self::Error(Status::NotAcceptable, err)
    }
}

impl<T, E> RResult<T, E> {
    pub fn ok(data: T) -> Self {
        Self::new_success(data)
    }
    pub fn err(err: E) -> Self {
        Self::new_err(err)
    }
    pub fn status_err(status: Status, err: E) -> Self {
        Self::new_status_err(status, err)
    }

    pub fn change_status(self, status: Status) -> Self {
        match self {
            RResult::Error(_, e) => Self::new_status_err(status, e),
            s => s,
        }
    }
}

impl<T, E> RResult<Wrap<T>, E> {
    pub fn wrap_ok(data: T) -> Self {
        Self::ok(Wrap(data))
    }
}

impl<T, E> RResult<T, E> {
    pub fn from_result(r: Result<T, E>) -> Self {
        r.into()
    }
    pub fn from_status_result(r: Result<T, E>, status: Status) -> Self {
        Self::from_result(r).change_status(status)
    }

    pub fn from_option<M: AsRef<str>>(o: Option<T>, msg: &M) -> RResult<T> {
        (o, msg).into()
    }

    pub fn from_status_option<M: AsRef<str>>(o: Option<T>, msg: &M, status: Status) -> RResult<T> {
        Self::from_option(o, msg).change_status(status)
    }
}
