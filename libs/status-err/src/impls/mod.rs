use std::num::ParseIntError;

use ::mongodb::bson;
use axum::extract::{
    multipart::{MultipartError, MultipartRejection},
    rejection::{JsonRejection, PathRejection, QueryRejection},
};
use checker::prefabs::{
    json_obj_check::JsonObjError, no_remainder_checker::HasRemError,
    num_check::NonZeroUnsignedError, version_checker::VersionInvalidError,
};
use jsonwebtoken::errors::Error as JwtError;
use serde_json::Error as JsonError;
use tonic::transport;

use crate::{ErrPrefix, GenError, StatusErr, status_error};

mod mongodb;
mod redis;
mod sea_orm;
// io prefix
status_error!(
    std::io::Error
    => crate::generated_error::io_kind::IoError
);
// parse prefix
status_error!(
url::ParseError
    => crate::generated_error::parse_kind::UrlParseError
);

status_error!(
ParseIntError
    => crate::generated_error::parse_kind::ParseIntError
);

status_error!(
JwtError
    => crate::generated_error::parse_kind::JwtError
);

status_error!(
chrono::ParseError
    => crate::generated_error::parse_kind::ChronoParseError
);
status_error!(
std::string::FromUtf8Error
    => crate::generated_error::parse_kind::FromUtf8Error
);

status_error!(
    http::header::ToStrError
    => crate::generated_error::parse_kind::ToStrError
);

status_error!(
    http_02::header::ToStrError
    => crate::generated_error::parse_kind::ToStrError
);

status_error!(
    http::header::InvalidHeaderValue
    => crate::generated_error::parse_kind::InvalidHeaderValueError
);

status_error!(
    http_02::header::InvalidHeaderValue
    => crate::generated_error::parse_kind::InvalidHeaderValueError
);

status_error!(
    bson::de::Error
    => crate::generated_error::parse_kind::BsonDeError
);

// check prefix
status_error!(
    std::convert::Infallible[
    ErrPrefix::CHECKER,
    0x00_00
    ]->""
);
status_error!(
    range_limit::Error
    => crate::generated_error::checker_kind::RangeLimitError
);
status_error!(
JsonError
    => crate::generated_error::checker_kind::JsonError
);

status_error!(
JsonRejection
    => crate::generated_error::checker_kind::JsonRejectionError
);
status_error!(
PathRejection
    => crate::generated_error::checker_kind::PathRejectionError
);
status_error!(
QueryRejection
    => crate::generated_error::checker_kind::QueryRejectionError
);
status_error!(
bincode::Error
    => crate::generated_error::checker_kind::BincodeError
);
status_error!(
    MultipartRejection
    => crate::generated_error::checker_kind::MultipartRejectionError
);

status_error!(
    MultipartError
    => crate::generated_error::checker_kind::MultipartError
);

// authorized prefix
status_error!(
    bcrypt::BcryptError
    => crate::generated_error::unauthorized_kind::BcryptError
);

status_error!(
    reqwest::Error
    => crate::generated_error::not_found_kind::ReqwestError
);

status_error!(
    NonZeroUnsignedError
    => crate::generated_error::checker_kind::NonZeroUnsignedError
);

impl<const RHS: u64> StatusErr for HasRemError<RHS> {
    fn respond_msg(&self) -> std::borrow::Cow<'_, str> {
        crate::generated_error::checker_kind::HasRemError
            .description()
            .into()
    }

    fn prefix(&self) -> ErrPrefix {
        ErrPrefix::mark_only(
            crate::generated_error::checker_kind::HasRemError.mark(),
        )
    }

    fn code(&self) -> u16 {
        crate::generated_error::checker_kind::HasRemError.code()
    }

    fn http_code(&self) -> http::StatusCode {
        crate::generated_error::checker_kind::HasRemError.status_code()
    }
}

status_error!(
    JsonObjError
    => crate::generated_error::checker_kind::JsonObjError
);

status_error!(
    transport::Error
    => crate::generated_error::logger_report_kind::TransportError
);

status_error!(
    tonic::Status
    => crate::generated_error::logger_report_kind::TonicStatusError
);

status_error!(
    bson::oid::Error
    => crate::generated_error::checker_kind::BsonOidError
);

status_error!(
    VersionInvalidError
    => crate::generated_error::checker_kind::VersionInvalidError
);

status_error!(
    serde_qs::Error
    => crate::generated_error::parse_kind::SerdeQsError
);

status_error!(
    hmac::digest::InvalidLength
    => crate::generated_error::parse_kind::HmacInvalidLengthError
);

status_error!(
    core::fmt::Error
    => crate::generated_error::io_kind::FmtError
);

status_error!(
    bson::uuid::Error
    => crate::generated_error::parse_kind::BsonUuidError
);

status_error!(
    bson::ser::Error
    => crate::generated_error::checker_kind::BsonSerError
);
