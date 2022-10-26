use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use database_traits::get_connect::GetMutDatabaseConnect;
use redis::AsyncCommands;
use redis_connection::{DbConnectConfig, RedisConnect, RedisDatabase};
use tokio::time::sleep;

static IS_CONNECT: AtomicBool = AtomicBool::new(false);

async fn init_connect() {
    if !IS_CONNECT.load(Ordering::Relaxed) {
        database_traits::initial::connect_db::<RedisDatabase, _>(&MockCfg)
            .await
            .expect("Connect to redis failure");
        IS_CONNECT.store(true, Ordering::Relaxed)
    }
}

#[derive(Debug, serde::Deserialize)]
struct MockCfg;

impl DbConnectConfig for MockCfg {
    fn scheme(&self) -> &str { "redis" }

    fn host(&self) -> &str { "192.168.181.128" }

    fn port(&self) -> u16 { 6379 }

    fn db(&self) -> u8 { 15 }
}

#[tokio::test]
async fn test_connect() {
    init_connect().await;

    let mut redis = RedisConnect::new();

    let conn = redis.mut_connect().unwrap();
    conn.set("key", "foo").await.expect("error")
}
#[tokio::test]
async fn test_set_timeout() {
    init_connect().await;

    let mut redis = RedisConnect::new();

    let conn = redis.mut_connect().unwrap();
    let _: Option<()> =
        conn.set_ex("outtime", "foo-out", 20).await.expect("Error");
    // read
    let v: String = conn.get("outtime").await.expect("errror");
    let ttl: i32 = conn.ttl("outtime").await.expect("Error");
    println!("ttl: {ttl}");
    assert_eq!(v, "foo-out");

    // sleep 15s
    sleep(Duration::from_secs(15)).await;
    let v: String = conn.get("outtime").await.expect("errror");
    let ttl: i32 = conn.ttl("outtime").await.expect("Error");
    println!("ttl: {ttl}");
    assert_eq!(v, "foo-out");

    // sleep 8s
    sleep(Duration::from_secs(8)).await;

    let expired: Option<String> = conn.get("outtime").await.expect("Error");
    let ttl: i32 = conn.ttl("outtime").await.expect("Error");
    println!("ttl: {ttl}");
    assert_eq!(expired, None)
}
