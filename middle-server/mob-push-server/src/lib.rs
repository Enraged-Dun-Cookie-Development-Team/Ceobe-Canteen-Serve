mod push_manager;
mod push_models;
mod requester;
mod config;
mod error;
mod push_forward;
pub mod push_notify;

mod pushing_data;

use general_request_client::client::RequestClient;
pub use pushing_data::PushEntity;


pub use error::MobPushError;
pub use push_forward::{PushForward, Scheme};

pub async fn mob_push<I, Mid, C>(
    req: &RequestClient, content: C, user_list: I,
) -> Result<(), ()>
where
    I: IntoIterator<Item = Mid>,
    Mid: AsRef<str>,
    C: PushEntity,
{
    todo!()
}
