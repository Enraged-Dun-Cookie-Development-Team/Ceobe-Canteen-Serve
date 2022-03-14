use std::{borrow::Cow};

pub trait RespError {
    fn description(&self) -> Cow<'static, str>;

    #[cfg(feature = "log")]
    fn do_logger(&self) {
        logger::error!("Error Ocurred : {}", self.description())
    }

    fn http_code(&self) -> http::StatusCode {
        http::StatusCode::INTERNAL_SERVER_ERROR
    }

    #[cfg(feature = "extra-code")]
    type ExtraCode: serde::Serialize + 'static + Sized+std::fmt::Display;
    #[cfg(feature = "extra-code")]
    fn extra_code(&self) -> Self::ExtraCode;
}


