use http::StatusCode;
use status_err::{status_error, ErrPrefix};

use crate::{error_generate, utils::req_pretreatment::prefabs::PathError};

error_generate!(
    pub MansionError
    Orm=sea_orm::DbErr
    Id=UnknownId
    NotFound=MansionNotFound
    Fraction=BadFraction
    Range=range_limit::Error
    Path=PathError
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
        1146: StatusCode::NOT_FOUND
    ]=>"指定饼学大厦ID未找到"
}

status_error! {
    pub BadFraction
    [
        ErrPrefix::CHECKER,
        0003: StatusCode::NOT_FOUND
    ]=>"错误的Fraction值范围(0~5)"
}
