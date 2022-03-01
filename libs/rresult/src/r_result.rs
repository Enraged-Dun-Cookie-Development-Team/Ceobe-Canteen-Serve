use crate::Wrap;

#[derive(Debug, PartialEq, Eq)]
pub enum RResult<T, E = String> {
    Success(T),
    Error(E),
}

impl<T, E> RResult<T, E> {
    fn new_success(data: T) -> Self {
        Self::Success(data)
    }

    fn new_err(err: E) -> Self {
        Self::Error(err)
    }
}

impl<T, E> RResult<T, E> {
    pub fn ok(data: T) -> Self {
        Self::new_success(data)
    }
    pub fn err(err: E) -> Self {
        Self::new_err(err)
    }
}

impl<T, E> RResult<Wrap<T>, E> {
    pub fn wrap_ok(data: T) -> Self {
        Self::ok(Wrap(data))
    }
}
