use crate::utils::req_pretreatment::prefabs::JsonError;

crate::error_generate!(
   pub CeobeError
    MailBox=actix::dev::MailboxError
    ActixWeb=actix_web::error::Error
    Json=JsonError
    Update=NoUpdateError
);

status_err::status_error!(
    pub NoUpdateError[
    status_err::ErrPrefix::NOT_MODIFIED,
    0001    
    ]=>"没有更新的Ceobe"
);

