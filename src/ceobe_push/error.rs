crate::error_generate!(
   pub CeobeError
    MailBox=actix::dev::MailboxError
    Update=NoUpdateError
);

crate::error_generate!(
    pub NoUpdateError="Ceobe No Updated"
);
