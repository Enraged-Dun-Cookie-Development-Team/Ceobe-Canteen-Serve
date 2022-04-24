use std::borrow::Cow;

pub(super) type DataSourceFilter = Vec<(u64, Cow<'static, str>)>;
