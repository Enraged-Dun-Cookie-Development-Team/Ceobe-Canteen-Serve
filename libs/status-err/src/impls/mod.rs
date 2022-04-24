mod mongodb;
mod sea_orm;
use std::{convert::Infallible, num::ParseIntError};

use http::StatusCode;

use crate::{status_error, ErrPrefix};

// actix prefix
status_error!(actix::dev::MailboxError[ErrPrefix::ACTIX , 0001: StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(actix_web::error::Error[ErrPrefix::ACTIX, 0002:StatusCode::INTERNAL_SERVER_ERROR]);
// io prefix
status_error!(std::io::Error[ErrPrefix::IO, 0001:StatusCode::INTERNAL_SERVER_ERROR]);
// parse prefix
status_error!(url::ParseError[ErrPrefix::PARSE, 0001:StatusCode::NOT_ACCEPTABLE]);
status_error!(ParseIntError[ErrPrefix::PARSE ,0002:StatusCode::NOT_ACCEPTABLE]);
status_error!(jwt::Error[ErrPrefix::PARSE,0003]);
status_error!(chrono::ParseError[ErrPrefix::PARSE, 0004]);
// check prefix
status_error!(Infallible[ErrPrefix::CHECKER,0000]);
status_error!(range_limit::Error[ErrPrefix::CHECKER ,0001:StatusCode::NOT_ACCEPTABLE]);
status_error!(request_pretreat::prefabs::JsonError[ErrPrefix::CHECKER,0004]);
status_error!(request_pretreat::prefabs::PathError[ErrPrefix::CHECKER,0005]);
status_error!(actix_web::error::QueryPayloadError[ErrPrefix::CHECKER,0007]);

// authorized prefix
status_error!(bcrypt::BcryptError[ErrPrefix::UNAUTHORIZED,0005:StatusCode::INTERNAL_SERVER_ERROR]);
