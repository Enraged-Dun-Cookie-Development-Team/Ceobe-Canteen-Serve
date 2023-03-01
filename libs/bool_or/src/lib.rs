/// 将 [`bool`](bool) 映射到 [`Result<(), E>`](Result<(),E>)
pub trait FalseOrError: Sized {
    #[inline]
    fn false_or<E>(self, e: E) -> Result<(), E> {
        <Self as FalseOrError>::false_or_with(self, || e)
    }

    fn false_or_with<E, F: FnOnce() -> E>(self, f: F) -> Result<(), E>;
}

impl FalseOrError for bool {
    #[inline]
    fn false_or_with<E, F: FnOnce() -> E>(self, f: F) -> Result<(), E> {
        match self {
            true => Err(f()),
            false => Ok(()),
        }
    }
}

/// 将 [`bool`](bool) 映射到 [`Result<(), E>`](Result<(),E>)
pub trait TrueOrError: Sized {
    #[inline]
    fn true_or<E>(self, e: E) -> Result<(), E> {
        <Self as TrueOrError>::true_or_with(self, || e)
    }

    fn true_or_with<E, F: FnOnce() -> E>(self, f: F) -> Result<(), E>;
}

impl TrueOrError for bool {
    #[inline]
    fn true_or_with<E, F: FnOnce() -> E>(self, f: F) -> Result<(), E> {
        match self {
            true => Ok(()),
            false => Err(f()),
        }
    }
}
