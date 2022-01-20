use std::sync::Arc;

use futures::StreamExt;
use tokio::runtime;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;

#[macro_use]
extern crate serde;

const DUN_BACKEND: &str = "ws://127.0.0.1/";
const PUSH_URL: &str = "http://localhost";
fn main() {
    // 最简单异步服务
    let rt = runtime::Builder::new_multi_thread()
        .max_blocking_threads(32)
        .enable_all()
        .build()
        .expect("Create Async Runtime Failure");

    rt.block_on(task())
}

async fn task() {
    // 连接到ws服务器
    let (mut socket, _) = connect_async(Url::parse(DUN_BACKEND).unwrap())
        .await
        .expect("Can not Connect To Ws Server");

    // 广播分发
    let url = Url::parse(PUSH_URL).unwrap();

    let client = Arc::new(
        reqwest::Client::builder()
            .referer(true)
            .build()
            .expect("Create http Client Failure"),
    );

    while let Some(Ok(msg)) = socket.next().await {
        let url = url.clone();
        let lclinet = Arc::clone(&client);
        tokio::spawn(async move {
            if let Message::Text(t) = msg {
                let _ = lclinet.post(url.clone()).body(t).send().await;
            } else if let Message::Binary(b) = msg {
                let _ = lclinet.post(url.clone()).body(b).send().await;
            }
        });
    }
}

fn default_top() -> bool {
    false
}

#[derive(Deserialize, Debug,Serialize)]
struct DataItem {
    #[serde(rename = "dataSource")]
    data_source: String,

    id: String,
    #[serde(rename = r#"timeForSort"#)]
    time_for_sort: u64,
    #[serde(rename = r#"timeForDisplay"#)]
    time_for_display: String,

    content: String,
    #[serde(rename = r#"jumpUrl"#)]
    jump_url: String,
    #[serde(rename = r#"coverImage"#)]
    cover_image: Option<String>,
    #[serde(rename = r#"imageList"#)]
    image_list: Vec<String>,
    #[serde(rename = r#"imageHttpList"#)]
    image_http_list: Vec<String>,
    #[serde(rename = r#"isTop"#, default = "default_top")]
    is_top: bool,

    retweeted: Option<serde_json::Value>,
    #[serde(rename = r#"componentData"#)]
    component_data: Option<serde_json::Value>,
}

// 判断其中一个数据源更新状态
fn check_update<T>(last_id: T, income: Vec<DataItem>) -> Option<Vec<DataItem>>
where
    T: AsRef<str>,
{
    if income.len() > 1 {
        let first = unsafe { income.get_unchecked(0) };
        if first.id != last_id.as_ref() {
            let mut res = Vec::with_capacity(income.len());
            let mut find_last=false;
            for inner in income {
                if inner.id != last_id.as_ref() {
                    res.push(inner)
                }else {
                    find_last=true;
                    break;
                }
            }
            match find_last{
                true => Some(res),
                false => Some(vec![res.into_iter().next().unwrap()]),
            }
        } else {
            None
        }
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_des() {
        let json = r##" {
        "dataSource": "官方微博",
        "id": "官方微博_6279793937/LaUiirubj",
        "timeForSort": 1642248901000,
        "timeForDisplay": "2022-01-15 20:15:01",
        "content": "#明日方舟#\n辞旧迎新，SideStory「将进酒」限时活动即将开启\n\n一、全新活动SideStory「将进酒」，活动关卡开启\n活动说明：活动期间将开放「将进酒」活动关卡，玩家可通过活动关卡作战、完成相关活动任务以及活动商店获取相关活动奖励\n解锁条件：通关主线1-10\n\n活动关卡将进行分段式开启：\n◆“三山奇闻 ...",
        "jumpUrl": "https://weibo.com/6279793937/LaUiirubj",
        "imageList": [],
        "imageHttpList": []
    }"##;

        let data = serde_json::from_str::<DataItem>(json).unwrap();
        println!("{:#?}", data);

        let a = String::from("2022-01-15 20:15:01");
        let b = "2022-01-15 20:15:02";
        assert_ne!(a, b);

        let str_prttey=serde_json::to_string_pretty(&data).unwrap();
        let str_=serde_json::to_string(&data).unwrap();

        println!("{}\n{}",str_,str_prttey)
    }
}
