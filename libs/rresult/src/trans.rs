
use crate::RResult;

impl<T, E> From<Result<T, E>> for RResult<T, E> {
    fn from(src: Result<T, E>) -> Self {
        match src {
            Ok(d) => Self::ok(d),
            Err(err) => Self::err(err),
        }
    }
}

#[derive(Debug)]
pub struct OptionNoneError<'s>(&'s str);
impl<'s> std::error::Error for OptionNoneError<'s> {}

impl<'s> std::fmt::Display for OptionNoneError<'s> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Option::None | {}", self.0)
    }
}

impl<'a, T, M> From<(Option<T>, &M)> for RResult<T>
where
    M: AsRef<str>,
{
    fn from((src, msg): (Option<T>, &M)) -> Self {
        match src {
            Some(data) => Self::ok(data),
            None => Self::err(msg.as_ref().to_string()),
        }
    }
}

impl<T, E> Into<Result<T, E>> for RResult<T, E> {
    fn into(self) -> Result<T, E> {
        match self {
            RResult::Success(da) => Ok(da),
            RResult::Error(_, e) => Err(e),
        }
    }
}

impl<T, E> Into<Option<T>> for RResult<T, E> {
    fn into(self) -> Option<T> {
        match self {
            RResult::Success(d) => Some(d),
            RResult::Error(_, _) => None,
        }
    }
}

#[cfg(feature = "outcome")]
impl<T, E> Into<rocket_::outcome::Outcome<T, (), ()>> for RResult<T, E> {
    fn into(self) -> rocket_::outcome::Outcome<T, (), ()> {
        match self {
            RResult::Success(d) => rocket_::outcome::Outcome::Success(d),
            RResult::Error(_, _) => rocket_::outcome::Outcome::Forward(()),
        }
    }
}
