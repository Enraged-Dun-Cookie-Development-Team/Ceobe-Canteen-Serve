use std::borrow::Cow;

crate::generate_model_register!(
    CeobePushModel
);

pub(super) type DataSourceFilter = Vec<(u64, Cow<'static, str>)>;

