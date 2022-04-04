use std::{future::Future, any::type_name, time::SystemTime};


pub async fn async_time_usage<F,Fut,R>(func:F)->R
where F:FnOnce()->Fut,
Fut:Future<Output = R>
{
    let name = type_name::<F>();
    let start = SystemTime::now();
    log::trace!("开始执行异步操作 {}", &name);
    let r = func().await;
    log::trace!("完成执行异步操作 {}", name);
    let end = SystemTime::now();
    let usage = end.duration_since(start).unwrap();
    log::debug!("执行异步操作 {} 用时 {}ms", name, usage.as_millis());
    r
}