use std::time::Duration;

use bytes::Bytes;
use futures::TryFutureExt;
use reqwest::Client;
use sql_models::ceobe_operation::video::bv;
use tokio::{
    spawn,
    sync::{mpsc, oneshot},
    task::yield_now,
};
use url::Url;

/// 独立执行的请求client.
///
/// bilibili 接口专用
///
/// 每次发送间隔为 500ms
pub(super) async fn idle_client(
    base_url: Url, client: Client,
    mut recv: mpsc::Receiver<(
        bv::Checked,
        oneshot::Sender<Result<Bytes, reqwest::Error>>,
    )>,
) {
    let mut interval = tokio::time::interval(Duration::from_millis(500));
    // 循环接受队列内容
    while let Some((bv, feedback)) = recv.recv().await {
        // 构造请求url,添加query [bvid]
        let mut url = base_url.clone();
        url.query_pairs_mut()
            .clear()
            .append_pair("bvid", bv.as_str());

        // 构造Future, 并载独立携程中执行
        let request = client.get(url).send().and_then(|resp| resp.bytes());
        spawn(async move {
            // 回调
            feedback.send(request.await).ok();
        });
        // 让出当前协程，使新协程立即被调度
        yield_now().await;

        // 延时500ms
        interval.tick().await;
    }
}
