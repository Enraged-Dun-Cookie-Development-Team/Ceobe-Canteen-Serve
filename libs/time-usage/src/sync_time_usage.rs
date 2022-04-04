use std::{any::type_name, time::SystemTime};

pub fn sync_time_usage<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let name = type_name::<F>();
    sync_time_usage_with_name(name, f)
}
pub fn sync_time_usage_with_name<F, R>(name: & str, f: F) -> R
where
    F: FnOnce() -> R,
{
    let start = SystemTime::now();
    log::trace!("开始执行同步操作 `{}`", &name);
    let r = f();
    log::trace!("完成执行同步操作 `{}`", name);
    let end = SystemTime::now();
    let usage = end.duration_since(start).unwrap();
    log::debug!("执行同步操作 `{}` 用时 {}ms", name, usage.as_millis());
    r
}
