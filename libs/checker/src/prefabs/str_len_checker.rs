use std::{marker::PhantomData, ops::Deref};

use futures::future::{ready, Ready};
use range_limit::{
    limits::{
        double_end_limit::DoubleEndLimit, fix_size::FixedSize,
        max_limit::MaxLimit, min_limit::MinLimit,
    },
    Error, RangeBound,
};

use crate::RefChecker;

/// 检查字符串的字符长度（而不是byte 长度）是否符合要求
///
/// 使用 [chars](str::chars) 构造迭代器，可参考其文档
pub struct StrCharLenChecker<S, Bound>(PhantomData<(S, Bound)>)
where
    S: Deref<Target = str> + 'static,
    Bound: RangeBound + 'static;

/// 检查字符串最大字符长度
pub type StrMaxCharLenChecker<S, const M: usize> =
    StrCharLenChecker<S, MaxLimit<M>>;
/// 检查字符串最小字符长度
pub type StrMinCharLenChecker<S, const M: usize> =
    StrCharLenChecker<S, MinLimit<M>>;
/// 检查字符串严格长度
pub type StrFixedCharLenChecker<S, const SIZE: usize> =
    StrCharLenChecker<S, FixedSize<SIZE>>;
/// 检查字符串字符长度是否在指定范围内
pub type StrCharLenRangeChecker<S, const L: usize, const U: usize> =
    StrCharLenChecker<S, DoubleEndLimit<L, U>>;

impl<S, Bound> RefChecker for StrCharLenChecker<S, Bound>
where
    S: Deref<Target = str> + 'static,
    Bound: RangeBound + 'static,
{
    type Args = ();
    type Err = Error;
    type Fut = Ready<Result<(), Self::Err>>;
    type Target = S;

    fn ref_checker(_: Self::Args, target: &Self::Target) -> Self::Fut {
        let size = target.chars().count();
        let result = Bound::match_range(size);
        ready(result.to_result(size))
    }
}

#[cfg(test)]
mod test {
    use range_limit::limits::max_limit::MaxLimit;

    use crate::{
        prefabs::str_len_checker::StrCharLenChecker, ToCheckRequire,
    };

    #[tokio::test]
    async fn test_checker() {
        // 6
        let a = "哇啊娃娃啊啊";
        assert_eq!(a.len(), 18);

        let checked = a
            .require_check::<StrCharLenChecker<_, MaxLimit<7>>>()
            .lite_checking()
            .await;

        assert_eq!(Ok(a), checked)
    }
}
