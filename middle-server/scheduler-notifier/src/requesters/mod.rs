use std::marker::PhantomData;

use general_request_client::{
    traits::{RequestBuilder, Requester},
    Method, Version,
};
use url::Url;

pub mod notify_platform_update;

/// 向调度器发送 notify 的路径
/// 编译期锁定
pub trait NotifyPath {
    const PATH: &'static str;
}

/// notify requester 创建
///
/// 通过 实现 [`NotifyRequester::create`] 创建 [`Requester`] 实例
/// 其中的参数 `url` 已经拼接完毕，可直接使用
pub trait NotifyRequester: Sized + Requester {
    /// 构造requester 时需要的额外参数
    type Args;

    /// 通过 [`Url`] 和 [`Self::Args`] 来构造 [`Requester`]
    fn create(url: Url, args: Self::Args) -> Self;

    /// 类似于 [`create`](Self::create) 但是 `url` 是 **base_url**
    ///
    /// # Note
    /// 该方法通常使用默认实现即可
    fn create_with_base_url(mut base_url: Url, args: Self::Args) -> Self
    where
        Self: NotifyPath,
    {
        base_url.set_path(<Self as NotifyPath>::PATH);
        <Self as NotifyRequester>::create(base_url, args)
    }
}

/// 特殊的requester, 可以使用 `P`覆盖 `R` 的 [`NotifyPath`]
pub struct PathOverwriteRequester<R: NotifyRequester, P: NotifyPath> {
    requester: R,
    __phantom: PhantomData<P>,
}

impl<R: NotifyRequester, P: NotifyPath> Requester
    for PathOverwriteRequester<R, P>
{
    const METHOD: Method = R::METHOD;
    const VERSION: Version = R::VERSION;

    fn get_url(&self) -> Url { self.requester.get_url() }

    fn prepare_request<B: RequestBuilder>(
        self, builder: B,
    ) -> Result<B::Request, B::Error> {
        self.requester.prepare_request(builder)
    }
}

impl<R: NotifyRequester, P: NotifyPath> NotifyRequester
    for PathOverwriteRequester<R, P>
{
    type Args = R::Args;

    fn create(url: Url, args: Self::Args) -> Self {
        Self {
            requester: <R as NotifyRequester>::create(url, args),
            __phantom: PhantomData,
        }
    }
}

impl<R: NotifyRequester, P: NotifyPath> NotifyPath
    for PathOverwriteRequester<R, P>
{
    const PATH: &'static str = P::PATH;
}
