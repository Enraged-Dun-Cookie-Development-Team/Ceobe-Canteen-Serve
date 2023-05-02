use mob_push_server::{MobPushConfigTrait, PushManager};

struct Cfg;

include!("../../../mob_push_cfg.meta");

#[tokio::test]
async fn task() {
    let client =
        general_request_client::axum_starter::request_client_prepare()
            .expect("init client failure")
            .0;
    let manager = mob_push_server::axum_starter::init_mob_push(&Cfg).await.0;

    let manager = PushManager::new_from_state(manager, client);

    let ret = manager
        .fetch_device_info("65l05lvwtep0fls")
        .await
        .expect("bad mid");

    println!("{ret:?}")
}
