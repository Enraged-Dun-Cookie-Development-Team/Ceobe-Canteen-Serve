use axum_starter::{state::AddState, prepare};
use tokio::{sync::mpsc, spawn};
use url::Url;
use super::{sender::QueryBiliVideo, idle_wroker::idle_client};

#[prepare(box BiliClientPrepare?)]
pub fn prepare_bili_client() -> Result<AddState<QueryBiliVideo>, PrepareError>
{
    let client = reqwest::Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:102.0) \
             Gecko/20100101 Firefox/102.0",
        )
        .build()?;

    let base_url =
        Url::parse("https://api.bilibili.com/x/web-interface/view")?;
    let (send, recv) = mpsc::channel(8);

    spawn(idle_client(base_url, client, recv));

    Ok(AddState::new(QueryBiliVideo::new(send)))
}
#[derive(Debug, thiserror::Error)]
pub enum PrepareError {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
}
