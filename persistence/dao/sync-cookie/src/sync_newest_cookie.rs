use db_ops_prelude::{
    get_connect::GetMutDatabaseConnect, mongodb::bson::oid::ObjectId,
};
use redis::AsyncCommands;
use redis_global::redis_key::{concat_key, cookie_list::CookieListKey};

use super::Result;
impl super::SyncCookieOperate {
    pub async fn sync_cookie(
        &mut self, cookie_id: Option<ObjectId>,
        update_cookie_id: Option<ObjectId>, comb_id: String,
        datasource: Option<String>,
    ) -> Result<()> {
        let redis = self.0.mut_connect();
        let cookie_id = if let (Some(mut newest_cookie_id), true) = (
            cookie_id,
            redis
                .hexists(CookieListKey::NEWEST_COOKIES, &comb_id)
                .await?,
        ) {
            let last_cookie_id: String =
                redis.hget(CookieListKey::NEWEST_COOKIES, &comb_id).await?;
            let last_cookie_id = last_cookie_id.parse()?;
            newest_cookie_id = newest_cookie_id.max(last_cookie_id);
            Some(newest_cookie_id.to_string())
        }
        else {
            cookie_id.map(|id| id.to_string())
        };

        if cookie_id.is_some() {
            redis
                .hset(CookieListKey::NEWEST_COOKIES, &comb_id, &cookie_id)
                .await?;
        }
        if let Some(update_id) = update_cookie_id {
            // 更新[更新最新饼id]到redis
            redis
                .set_nx(
                    concat_key(
                        CookieListKey::NEW_UPDATE_COOKIE_ID,
                        &update_id.to_string(),
                    ),
                    true,
                )
                .await?;
            if redis
                .hexists(CookieListKey::NEW_UPDATE_COOKIES, &datasource)
                .await?
            {
                let update_cookie: String = redis
                    .hget(CookieListKey::NEW_UPDATE_COOKIES, &datasource)
                    .await?;
                if update_id.to_string() != update_cookie {
                    // 对已经被替换下的饼id设置ttl，2小时
                    redis
                        .set_ex(
                            concat_key(
                                CookieListKey::NEW_UPDATE_COOKIE_ID,
                                &update_cookie,
                            ),
                            true,
                            2 * 60 * 60,
                        )
                        .await?;
                }
            }
            redis
                .hset(
                    CookieListKey::NEW_UPDATE_COOKIES,
                    &datasource,
                    &update_id.to_string(),
                )
                .await?;
        }

        Ok(())
    }
}
