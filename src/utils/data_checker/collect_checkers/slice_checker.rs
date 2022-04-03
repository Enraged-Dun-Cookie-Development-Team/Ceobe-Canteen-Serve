use std::marker::PhantomData;

use futures::Future;

use crate::utils::data_checker::DataChecker;

#[derive(Debug)]
pub struct SliceChecker<S, C, O>(PhantomData<(S, C, O)>)
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: DataChecker<Unchecked = S::Item>,
    C::Args: Clone;

impl<S, C, O> DataChecker for SliceChecker<S, C, O>
where
    S: IntoIterator,
    O: FromIterator<C::Checked>,
    C: DataChecker<Unchecked = S::Item>,
    C::Args: Clone,
{
    type Args = C::Args;
    type Checked = O;
    type Err = C::Err;
    type Unchecked = S;

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

            Ok(O::from_iter(temp.into_iter()))
        }
    }
}
