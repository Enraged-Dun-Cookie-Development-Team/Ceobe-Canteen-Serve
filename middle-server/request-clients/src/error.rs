use status_err::ErrPrefix;

#[derive(Debug, thiserror::Error)]
#[error("管道发生未预期关闭")]
pub struct ChannelClose;

impl status_err::StatusErr for ChannelClose {
    fn prefix(&self) -> ErrPrefix { ErrPrefix::SERVE }

    fn code(&self) -> u16 { 0x0002 }
}
