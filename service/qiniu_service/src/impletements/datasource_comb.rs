use ceobe_qiniu_upload::QiniuManager;
use persistence::{operate::GetMutDatabaseConnect, redis::RedisConnect};
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};
use redis::AsyncCommands;
use redis_global::redis_key::cookie_list::CookieListKey;

use crate::{
    error::ServiceResult,
    model::{CombIdToCookieId, CombIdToCookieIdPlayLoad, DeleteObjectName},
    QiniuService,
};

impl QiniuService {
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

    #[deprecated]
    #[allow(deprecated)]
    /// 用于脚本的删除与上传最新饼id到七牛云
    pub async fn upload_newest_cookie_id_use_script(
        qiniu: QiniuManager, cookie_id: String,
        qq_channel: &mut QqChannelGrpcService, comb_id: String,
    ) {
        // 先删除，后新增
        let result = qiniu
            .delete(DeleteObjectName {
                file_name: comb_id.clone(),
            })
            .await
            .err();
        if let Some(err) = result {
            let _ = qq_channel
                .send_logger(
                    LogRequest::builder()
                        .level(LogType::Error)
                        .manual()
                        .info("删除七牛云数据源对应最新饼id文件失败".into())
                        .extra(format!("报错：{err}\n组合id：{comb_id}"))
                        .build(),
                )
                .await;
        }

        let source = CombIdToCookieId {
            cookie_id: Some(&cookie_id),
            update_cookie_id: None,
        };
        let payload = CombIdToCookieIdPlayLoad {
            file_name: &comb_id,
        };

        // 上传数据源组合到对象储存[重试3次]
        let mut result = Option::<ceobe_qiniu_upload::Error>::None;
        for _ in 0..3 {
            result = qiniu_cdn_upload::upload(&qiniu, &source, payload)
                .await
                .err();
            if result.is_none() {
                break;
            }
        }
        if let Some(err) = result {
            let _ = qq_channel
                .send_logger(
                    LogRequest::builder()
                        .level(LogType::Error)
                        .manual()
                        .info("上传七牛云数据源对应最新饼id文件失败".into())
                        .extra(format!(
                            "报错：{err}\n组合id：{comb_id}\n最新饼id：\
                             {cookie_id:#?}\n",
                        ))
                        .build(),
                )
                .await;
        }
    }
}
