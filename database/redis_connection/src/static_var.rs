use once_cell::sync::OnceCell;
use redis::{aio::ConnectionManager, RedisError};
use tracing::{info, instrument};
use url::Url;

use crate::config::DbConnectConfig;
static REDIS_DATABASE_CLIENT: OnceCell<ConnectionManager> = OnceCell::new();

#[instrument(skip(config))]
pub async fn connect_to_redis_database<C>(
    config: &C,
) -> Result<(), RedisError>
where
    C: DbConnectConfig,
{
    let mut url = Url::parse("redis://").unwrap();

    url.set_host(config.host().into()).unwrap();
    url.set_port(config.port().into()).unwrap();
    url.set_password(config.password()).unwrap();
    url.path_segments_mut()
        .unwrap()
        .extend([config.db().to_string()]);

    info!(redis.url = %url, redis.connect = true);
    let client = redis::Client::open(url)?;
    let manager = ConnectionManager::new(client).await?;
    if REDIS_DATABASE_CLIENT.set(manager).is_err() {
        panic!("Redis 数据库连接已经建立")
    }

    Ok(())
}

/// 获取redis数据库
pub fn get_redis_client() -> &'static ConnectionManager {
    REDIS_DATABASE_CLIENT.get().expect("Redis 数据库连接未建立")
}

#[cfg(test)]
mod test {
    use url::Url;

    #[test]
    fn test_url() {
        let mut url = Url::parse("redis://").expect("bad url");

        url.set_host("localhost".into()).expect("Cannot be base");
        url.set_password("localhost".into())
            .expect("Cannot be base");
        println!("{url:?}");
    }
}
