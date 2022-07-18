mod mongodb;
mod sea_orm;
use std::{convert::Infallible, num::ParseIntError};

use http::StatusCode;

use crate::{status_error, ErrPrefix};

// actix prefix
status_error!(actix::dev::MailboxError[ErrPrefix::ACTIX , 1: StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(actix_web::error::Error[ErrPrefix::ACTIX, 2:StatusCode::INTERNAL_SERVER_ERROR]);
// io prefix
status_error!(std::io::Error[ErrPrefix::IO, 1:StatusCode::INTERNAL_SERVER_ERROR]);
// parse prefix
status_error!(url::ParseError[ErrPrefix::PARSE, 1:StatusCode::NOT_ACCEPTABLE]);
status_error!(ParseIntError[ErrPrefix::PARSE ,2:StatusCode::NOT_ACCEPTABLE]);
status_error!(jwt::Error[ErrPrefix::PARSE,3]);
status_error!(chrono::ParseError[ErrPrefix::PARSE, 4]);
// check prefix
status_error!(Infallible[ErrPrefix::CHECKER,0]);
status_error!(range_limit::Error[ErrPrefix::CHECKER ,1:StatusCode::NOT_ACCEPTABLE]);
status_error!(request_pretreat::prefabs::JsonError[ErrPrefix::CHECKER,4]);
status_error!(request_pretreat::prefabs::PathError[ErrPrefix::CHECKER,5]);
status_error!(actix_web::error::QueryPayloadError[ErrPrefix::CHECKER,7]);

// authorized prefix
status_error!(
    bcrypt::BcryptError[
        ErrPrefix::UNAUTHORIZED,
        5:StatusCode::INTERNAL_SERVER_ERROR
    ]
);
