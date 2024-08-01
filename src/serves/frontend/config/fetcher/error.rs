use axum_resp_result::RespResult;

use crate::error_generate;

error_generate! {
    pub FechterError

    Base70ConvBitmap = bitmap_convert::error::Error
    RequestError = reqwest::Error
}

pub type FetcherRResult<T> = RespResult<T, FechterError>;
