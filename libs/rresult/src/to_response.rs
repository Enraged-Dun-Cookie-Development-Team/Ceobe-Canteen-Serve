use std::io::Cursor;

use rocket_::{
    http::{ContentType, Status},
    response::{self, Responder},
    Request, Response,
};

use crate::{IntoSerde, RResult};

impl<'r, 'o, T, E> Responder<'r, 'o> for RResult<T, E>
where
    'o: 'r,
    T: for<'s> IntoSerde<'s>,
    E: std::error::Error,
{
    fn respond_to(self, _request: &'r Request<'_>) -> response::Result<'o> {
        let t = serde_json::to_vec(&self).or_else(|_e| Err(Status::InternalServerError))?;
        let status = match self {
            RResult::Success(_) => Status::Ok,
            RResult::Error(s, _) => s,
        };

        #[cfg(feature = "logger")]
        log::info!(
            "Respond by RRsult | status: {}, content-size: {}",
            status,
            t.len()
        );

        Response::build()
            .header(ContentType::JSON)
            .sized_body(t.len(), Cursor::new(t))
            .status(status)
            .ok()
    }
}
