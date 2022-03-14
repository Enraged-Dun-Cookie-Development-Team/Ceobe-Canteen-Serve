use std::borrow::Cow;

pub(crate) trait OwnerLeaker {
    type Leak: 'static + Sized;
    fn leak(self) -> Self::Leak;
}

impl OwnerLeaker for String {
    type Leak = &'static str;

    fn leak(self) -> Self::Leak {
        let boxed = self.into_boxed_str();
        Box::leak(boxed) as &'static str
    }
}

impl<T> OwnerLeaker for Option<T>
where
    T: OwnerLeaker,
{
    type Leak = Option<T::Leak>;

    fn leak(self) -> Self::Leak {
        self.map(|e| e.leak())
    }
}

impl OwnerLeaker for Cow<'static, str> {
    type Leak = &'static str;

    fn leak(self) -> Self::Leak {
        match self {
            Cow::Borrowed(s) => s,
            Cow::Owned(s) => {
                let s = s.into_boxed_str();
                Box::leak(s) as &'static str
            }
        }
    }
}
