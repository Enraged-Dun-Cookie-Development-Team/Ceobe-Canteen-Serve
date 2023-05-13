use std::str::FromStr;

use ceobe_qiniu_upload::QiniuManager;
use mongodb::bson::oid::ObjectId;
use qiniu_cdn_upload::upload;
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};
use redis::AsyncCommands;
use redis_connection::{
    database_traits::get_connect::GetMutDatabaseConnect, RedisConnect,
};
use redis_global::redis_key::cookie_list::CookieListKey;
use tokio::task::JoinHandle;

use crate::{
    error::ServiceResult,
    model::{CombIdToCookieId, CombIdToCookieIdPlayLoad, DeleteObjectName},
    QiniuService,
};

impl QiniuService {
    /// 新增数据源组合对应最新饼id文件到对象存储
    pub async fn create_datasource_comb(
        qiniu: &QiniuManager, qq_channel: &mut QqChannelGrpcService,
        redis_client: &mut RedisConnect, mut cookie_id: Option<String>,
        update_cookie_id: Option<String>, comb_id: String,
        datasource: Option<String>,
    ) -> ServiceResult<()> {
        let redis = redis_client.mut_connect();
        // 获取该数据源组合目前最新的饼
        if cookie_id.is_some()
            && redis
                .hexists(CookieListKey::NEWEST_COOKIES, &comb_id)
                .await?
        {
            let last_cookie_id: String =
                redis.hget(CookieListKey::NEWEST_COOKIES, &comb_id).await?;
            let last_cookie_id = ObjectId::from_str(&last_cookie_id)?;
            let mut object_cookie_id =
                ObjectId::from_str(&cookie_id.unwrap())?;
            object_cookie_id = object_cookie_id.max(last_cookie_id);
            cookie_id = Some(object_cookie_id.to_string());
        }

        let source = CombIdToCookieId {
            cookie_id: cookie_id.clone(),
            update_cookie_id: update_cookie_id.clone(),
        };
        let payload = CombIdToCookieIdPlayLoad {
            file_name: &comb_id,
        };

        // 上传数据源组合到对象储存[重试3次]
        let mut result = Option::<ceobe_qiniu_upload::Error>::None;
        for _ in 0..3 {
            result = upload(qiniu, &source, payload).await.err();
            if result.is_none() {
                break;
            }
        }
        match result {
            Some(err) => {
                qq_channel
                    .send_logger(
                        LogRequest::builder()
                            .level(LogType::Error)
                            .manual()
                            .info(
                                "上传七牛云数据源对应最新饼id文件失败".into(),
                            )
                            .extra(format!(
                                "报错：{err}\n组合id：{comb_id}\n最新饼id：\
                                 {cookie_id:#?}\n更新饼id：\
                                 {update_cookie_id:#?}",
                            ))
                            .build(),
                    )
                    .await?;
                Err(err)?;
            }
            None => {
                if cookie_id.is_some() {
                    redis
                        .hset(
                            CookieListKey::NEWEST_COOKIES,
                            &comb_id,
                            &cookie_id,
                        )
                        .await?;
                }
                if let Some(update_id) = update_cookie_id {
                    // 更新[更新最新饼id]到redis
                    redis
                        .set_nx(
                            format!(
                                "{}{}",
                                CookieListKey::NEW_UPDATE_COOKIE_ID,
                                &update_id
                            ),
                            true,
                        )
                        .await?;
                    if redis
                        .hexists(
                            CookieListKey::NEW_UPDATE_COOKIES,
                            &datasource,
                        )
                        .await?
                    {
                        let update_cookie: String = redis
                            .hget(
                                CookieListKey::NEW_UPDATE_COOKIES,
                                &datasource,
                            )
                            .await?;
                        if update_id != update_cookie {
                            // 对已经被替换下的饼id设置ttl，2小时
                            redis
                                .set_ex(
                                    format!(
                                        "{}{}",
                                        CookieListKey::NEW_UPDATE_COOKIE_ID,
                                        update_cookie
                                    ),
                                    true,
                                    2 * 60 * 60,
                                )
                                .await?;
                        }
                        redis
                            .hset(
                                CookieListKey::NEW_UPDATE_COOKIES,
                                &datasource,
                                &update_id,
                            )
                            .await?;
                    }
                }
            }
        }
        Ok(())
    }

    /// 删除数据源组合对应最新饼id文件
    pub async fn delete_datasource_comb(
        qiniu: &QiniuManager, qq_channel: &mut QqChannelGrpcService,
        redis_client: &mut RedisConnect, comb_id: String,
    ) -> ServiceResult<()> {
        let result = qiniu
            .delete(DeleteObjectName {
                file_name: comb_id.clone(),
            })
            .await
            .err();
        match result {
            Some(err) => {
                qq_channel
                    .send_logger(
                        LogRequest::builder()
                            .level(LogType::Error)
                            .manual()
                            .info(
                                "删除七牛云数据源对应最新饼id文件失败".into(),
                            )
                            .extra(format!("报错：{err}\n组合id：{comb_id}"))
                            .build(),
                    )
                    .await?;
                Err(err)?;
            }
            None => {
                let redis = redis_client.mut_connect();
                redis.hdel(CookieListKey::NEWEST_COOKIES, &comb_id).await?;
            }
        }
        Ok(())
    }

    /// 删除数据源组合对应最新饼id文件,没有redis操作
    pub async fn delete_datasource_comb_without_redis(
        qiniu: &QiniuManager, qq_channel: &mut QqChannelGrpcService,
        comb_id: String,
    ) -> ServiceResult<()> {
        let result = qiniu
            .delete(DeleteObjectName {
                file_name: comb_id.clone(),
            })
            .await
            .err();
        if let Some(err) = result {
            qq_channel
                .send_logger(
                    LogRequest::builder()
                        .level(LogType::Error)
                        .manual()
                        .info("删除七牛云数据源对应最新饼id文件失败".into())
                        .extra(format!("报错：{err}\n组合id：{comb_id}"))
                        .build(),
                )
                .await?;
            Err(err)?;
        }
        Ok(())
    }

    /// 更新数据源组合文件（删除+新增）
    pub async fn update_datasource_comb(
        qiniu: QiniuManager, mut qq_channel: QqChannelGrpcService,
        mut redis_client: RedisConnect, cookie_id: Option<String>,
        update_cookie_id: Option<String>, comb_id: String,
        datasource: Option<String>,
    ) {
        if Self::delete_datasource_comb_without_redis(
            &qiniu,
            &mut qq_channel,
            comb_id.clone(),
        )
        .await
        .is_ok()
        {
            let _ = Self::create_datasource_comb(
                &qiniu,
                &mut qq_channel,
                &mut redis_client,
                cookie_id,
                update_cookie_id,
                comb_id,
                datasource,
            )
            .await
            .is_err();
        }
    }

    /// 批量更新数据源组合文件
    pub async fn update_multi_datasource_comb(
        qiniu: QiniuManager, cookie_id: Option<String>,
        update_cookie_id: Option<String>, qq_channel: QqChannelGrpcService,
        redis_client: RedisConnect, comb_ids: Vec<String>,
        datasource: Option<String>,
    ) {
        let mut handles = Vec::<JoinHandle<()>>::new();
        for comb_id in comb_ids {
            handles.push(tokio::spawn(Self::update_datasource_comb(
                qiniu.clone(),
                qq_channel.clone(),
                redis_client.clone(),
                cookie_id.clone(),
                update_cookie_id.clone(),
                comb_id,
                datasource.clone(),
            )));
        }
        futures::future::join_all(handles).await;
    }
}
