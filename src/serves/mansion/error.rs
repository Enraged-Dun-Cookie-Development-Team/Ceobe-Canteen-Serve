use actix_web::error::QueryPayloadError;
use http::StatusCode;
use status_err::{status_error, ErrPrefix};

use crate::{
    error_generate,
    utils::{
        mongodb_utils::error::MongoDbError,
        req_pretreatment::prefabs::{JsonError, PathError},
        user_authorize::error::AuthError,
    },
};

error_generate!(
    pub MansionError
    // request entity error
    Auth=AuthError
    Path=PathError
    Json=JsonError
    Query=QueryPayloadError
    //db error
    Mongo=MongoDbError
    // data parse error
    Date=chrono::ParseError
    // check error
    Range=range_limit::Error
    Id=UnknownId
    NotFound=MansionNotFound
    Fraction=BadFraction
    Predict=UnknownPredictType
    MansionExist=MansionIdExist
);

status_error! {
    pub UnknownId
    [
        ErrPrefix::CHECKER,
        0002: StatusCode::NOT_ACCEPTABLE
    ]=>"饼学大厦id格式不是 {int}.{int}"
}

status_error! {
    pub MansionNotFound
    [
        ErrPrefix::NOT_FOUND,
        0001: StatusCode::NOT_FOUND
    ]=>"指定饼学大厦ID未找到"
}

status_error! {
    pub MansionIdExist
    [
        ErrPrefix::CHECKER,
        0008: StatusCode::CONFLICT
    ]=>"指定ID的饼学大厦已经存在"
}

status_error! {
    pub BadFraction
    [
        ErrPrefix::CHECKER,
        0003: StatusCode::NOT_FOUND
    ]=>"错误的Fraction值范围(0~5)"
}

status_error! {
    pub UnknownPredictType
    [
        ErrPrefix::CHECKER,
    0006
    ]=>"未知的预期确信度等级"
}
