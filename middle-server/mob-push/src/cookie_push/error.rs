
#[derive(Debug,thiserror::Error)]

pub enum InternalError {
    #[error(transparent)]
    Fetcher(#[from]fetcher::datasource_config::OperateError),
    #[error(transparent)]
    UserProperty(#[from]ceobe_user::property::OperateError),
    #[error("推送器协程意外退出")]
    PusherDeath
}