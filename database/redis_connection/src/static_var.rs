use once_cell::sync::OnceCell;
use redis::{aio::ConnectionManager, RedisError};

use crate::config::DbConnectConfig;
static REDIS_DATABASE_CLIENT: OnceCell<ConnectionManager> = OnceCell::new();

pub async fn connect_to_redis_database<C>(
    config: &C,
) -> Result<(), RedisError>
where
    C: DbConnectConfig,
{
    let db_url = format!(
        "{scheme}://:{password}@{host}:{port}/{db}",
        scheme = config.scheme(),
        password = config.password(),
        host = config.host(),
        port = config.port(),
        db = config.db()
    );

    log::info!("准备连接到数据库: {}", db_url);
    let client = redis::Client::open(db_url)?;
    let manager = ConnectionManager::new(client).await?;
    if REDIS_DATABASE_CLIENT.set(manager).is_err() {
        panic!("Redis 数据库连接已经建立")
    }

    Ok(())
}

// 获取redis数据库
pub fn get_redis_client() -> &'static ConnectionManager {
    REDIS_DATABASE_CLIENT.get().expect("Redis 数据库连接未建立")
}
