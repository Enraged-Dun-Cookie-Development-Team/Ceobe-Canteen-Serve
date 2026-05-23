use status_err::{
    generated_error::serve_kind::ChannelCloseError, status_error,
};

#[derive(Debug, thiserror::Error)]
#[error("管道发生未预期关闭")]
pub struct ChannelClose;

status_error!(
    ChannelClose
    => ChannelCloseError
);
