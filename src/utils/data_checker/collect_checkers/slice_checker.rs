use std::marker::PhantomData;

use futures::Future;

use crate::utils::data_checker::DataChecker;

pub struct SliceChecker<S, C>(PhantomData<(S, C)>);

impl<S, C> DataChecker for SliceChecker<S, C>
where
    S: IntoIterator + FromIterator<C::Checked>,
    C: DataChecker<Unchecked = S::Item>,
    C::Args: Clone,
{
    type Unchecked = S;

    type Args = C::Args;

    type Checked = S;

    type Err = C::Err;

    type Fut = impl Future<Output = Result<Self::Checked, Self::Err>>;

    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut {
        async move {
            let iter = uncheck
                .into_iter()
                .map(|item| C::checker(args.clone(), item));

            let mut temp = vec![];

            for fut in iter {
                let resp = fut.await?;
                temp.push(resp);
            }

            Ok(S::from_iter(temp.into_iter()))
        }
    }
}
