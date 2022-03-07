use std::num::ParseIntError;

use http::StatusCode;

use crate::status_error;

status_error!(actix::dev::MailboxError['A' 0001: StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(actix_web::error::Error['A' 0002:StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(std::io::Error['I' 0001:StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(sea_orm::DbErr['D' 0001:StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(url::ParseError['P' 0001:StatusCode::NOT_ACCEPTABLE]);
status_error!(ParseIntError['P' 0002:StatusCode::INTERNAL_SERVER_ERROR]);
status_error!(range_limit::Error['C' 0001:StatusCode::INTERNAL_SERVER_ERROR]);
