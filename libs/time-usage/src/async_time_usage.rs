use std::{any::type_name, future::Future, time::SystemTime};

pub async fn async_time_usage<Fut, R>(fut: Fut) -> R
where
    Fut: Future<Output = R>,
{
    let name = type_name::<Fut>();
    async_time_usage_with_name(name, fut).await
}
pub async fn async_time_usage_with_name<Fut, R>(
    name: & str, fut: Fut,
) -> R
where
    Fut: Future<Output = R>,
{
    let start = SystemTime::now();
    log::trace!("开始执行异步操作 `{}`", &name);
    let r = fut.await;
    log::trace!("完成执行异步操作 `{}`", name);
    let end = SystemTime::now();
    let usage = end.duration_since(start).unwrap();
    log::debug!("执行异步操作 `{}` 用时 {}ms", name, usage.as_millis());
    r
}
