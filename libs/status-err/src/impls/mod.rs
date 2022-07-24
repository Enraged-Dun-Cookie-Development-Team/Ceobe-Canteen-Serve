mod mongodb;
mod sea_orm;
use std::{convert::Infallible, num::ParseIntError};

use axum::extract::rejection::{
    JsonRejection, PathRejection, QueryRejection,
};
use http::StatusCode;

use crate::{status_error, ErrPrefix};

// io prefix
status_error!(std::io::Error[ErrPrefix::IO, 1:StatusCode::INTERNAL_SERVER_ERROR]);
// parse prefix
status_error!(url::ParseError[ErrPrefix::PARSE, 1:StatusCode::NOT_ACCEPTABLE]);
status_error!(ParseIntError[ErrPrefix::PARSE ,2:StatusCode::NOT_ACCEPTABLE]);
status_error!(jwt::Error[ErrPrefix::PARSE,3]);
status_error!(chrono::ParseError[ErrPrefix::PARSE, 4]);
status_error!(std::string::FromUtf8Error[ErrPrefix::PARSE, 5:StatusCode::INTERNAL_SERVER_ERROR]);
// check prefix
status_error!(Infallible[ErrPrefix::CHECKER, 0x00_00]);
status_error!(range_limit::Error[
ErrPrefix::CHECKER,
0x00_01:StatusCode::NOT_ACCEPTABLE
]);
status_error!(JsonRejection[ErrPrefix::CHECKER, 0x00_04]);
status_error!(PathRejection[ErrPrefix::CHECKER, 0x00_05]);
status_error!(QueryRejection[ErrPrefix::CHECKER, 0x00_07]);

// authorized prefix
status_error!(
    bcrypt::BcryptError[
        ErrPrefix::UNAUTHORIZED,
        5:StatusCode::INTERNAL_SERVER_ERROR
    ]
);

status_error!(
    reqwest::Error[
        ErrPrefix::NOT_FOUND,
        3 : StatusCode::INTERNAL_SERVER_ERROR
    ]
);
