use axum_starter::{prepare, state::AddState};
use reqwest::Error;

use crate::client::RequestClient;

#[prepare(RequestClient?)]
pub fn request_client_prepare() -> Result<AddState<RequestClient>, Error> {
    Ok(AddState::new(RequestClient::new_with(|builder| {
        builder.user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) \
            Gecko/20100101 Firefox/102.0",
        )
    })?))
}
