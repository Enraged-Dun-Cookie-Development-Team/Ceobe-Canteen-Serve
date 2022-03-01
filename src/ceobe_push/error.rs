crate::error_generate!(
   pub CeobeError
    MailBox=actix::dev::MailboxError
    Update=NoUpdateError
);

crate::error_generate!(
    pub NoUpdateError="Ceobe No Updated"
);

rresult::coded_error!(NoUpdateError[4005:http::StatusCode::NOT_MODIFIED]);

