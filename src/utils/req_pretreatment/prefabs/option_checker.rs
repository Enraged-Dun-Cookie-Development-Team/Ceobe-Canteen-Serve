use std::marker::PhantomData;

use futures::Future;

use crate::utils::data_checker::DataChecker;

pub struct OptionChecker<C: DataChecker>(PhantomData<C>);

impl<C: DataChecker> DataChecker for OptionChecker<C> {
    type Unchecked = Option<C::Unchecked>;

    type Args = C::Args;

    type Checked = Option<C::Checked>;

    type Err = C::Err;

    type Fut = impl Future<Output = Result<Self::Checked, Self::Err>>;

    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        async move {
            match uncheck {
                Some(uc) => C::checker(args, uc).await.map(Some),
                None => Ok(Self::Checked::None),
            }
        }
    }
}
