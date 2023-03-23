pub mod axum_starter;
mod config;
mod error;
pub mod push_forward;
mod push_manager;
mod push_models;
pub mod push_notify;
mod requester;

mod pushing_data;
pub use config::app_info::MobPushConfigTrait;
pub use error::MobPushError;
use general_request_client::client::RequestClient;
pub use push_forward::{PushForward, Scheme};
pub use push_manager::PushManager;
pub use pushing_data::PushEntity;

use crate::push_models::response::Respond;

/// 通过使用给定的request 客户端，发起mob推送.
///
/// - 推送内容使用 `content` 定义
/// - 推送用户通过 `user_list` 提供
///
/// # Errors
///
/// This function will return an error if
/// - 构造 requester 时，json 序列化异常
/// - 发送请求时异常
/// - 读取响应体时异常
/// - 反序列响应体时异常
/// - MobPush 响应的推送异常
pub async fn mob_push<'mid, I, Mid, C>(
    client: &RequestClient, manager: &mut PushManager, content: &C,
    user_list: I,
) -> Result<(), crate::error::MobPushError>
where
    I: IntoIterator<Item = &'mid Mid>,
    Mid: AsRef<str> + 'mid,
    C: PushEntity,
{
    let users = user_list
        .into_iter()
        .map(AsRef::<str>::as_ref)
        .collect::<Vec<_>>();
    let mut delayer = manager.batch_delay();
    let requester_iter = manager.new_requester(&users, content);

    delayer.delay().await;
    for requester in requester_iter {
        let requester = requester?;

        let resp = client.send_request(requester).await?;

        let _resp = serde_json::from_slice::<Respond>(&resp.bytes().await?)?
            .to_result()?;

        delayer.delay().await;
    }

    Ok(())
}
