use std::time::Duration;

use bytes::Bytes;
use futures::TryFutureExt;
use reqwest::Client;
use sql_models::ceobe_operation::video::checkers::bv::Bv;
use tokio::{sync::{mpsc, oneshot}, spawn, task::yield_now};
use url::Url;

pub(super) async fn idle_client(
    base_url: Url, client: Client,
    mut recv: mpsc::Receiver<(
        Bv,
        oneshot::Sender<Result<Bytes, reqwest::Error>>,
    )>,
) {
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    while let Some((bv, feedback)) = recv.recv().await {
        let mut url = base_url.clone();
        url.query_pairs_mut()
            .clear()
            .append_pair("bvid", bv.as_str());

        let request = client.get(url).send().and_then(|resp| resp.bytes());
        spawn(async move {
            feedback.send(request.await).ok();
        });
        yield_now().await;

        interval.tick().await;
    }
}
