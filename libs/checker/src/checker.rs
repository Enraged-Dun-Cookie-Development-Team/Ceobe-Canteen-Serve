use std::future::Future;

pub trait Checker {
    /// 未经过检查时的值
    type Unchecked;
    /// 检查时需要的外部信息
    type Args;
    /// 通过检查的值
    type Checked;
    /// 检查过程中出现的异常
    type Err;

    /// 检查过程可能为异步
    type Fut: Future<Output = Result<Self::Checked, Self::Err>>;

    /// 进行数据检查，可能为异步
    fn checker(args: Self::Args, uncheck: Self::Unchecked) -> Self::Fut;
}

pub trait RefChecker: 'static {
    type Target;

    type Err;

    type Args;

    type Fut: Future<Output = Result<(), Self::Err>>;

    fn ref_checker(args: Self::Args, target: &Self::Target) -> Self::Fut;
}

pub trait LiteChecker {
    /// 未经过检查时的值
    type Unchecked;
    /// 通过检查的值
    type Checked;
    /// 检查过程中出现的异常
    type Err;

    /// 检查过程可能为异步
    type Fut: Future<Output = Result<Self::Checked, Self::Err>>;

    /// 进行数据检查，可能为异步
    fn checker(uncheck: Self::Unchecked) -> Self::Fut;
}

pub trait LiteRefChecker {
    type Target;

    type Err;

    type Fut: Future<Output = Result<(), Self::Err>>;

    fn ref_checker(target: &Self::Target) -> Self::Fut;
}
