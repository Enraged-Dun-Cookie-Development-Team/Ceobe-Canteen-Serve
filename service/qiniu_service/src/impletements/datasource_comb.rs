use ceobe_qiniu_upload::QiniuManager;
use qiniu_cdn_upload::upload;
use qq_channel_warning::{LogRequest, LogType, QqChannelGrpcService};
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
        cookie_id: Option<String>, update_cookie_id: Option<String>, comb_id: String,
    ) -> ServiceResult<()> {
        let source = CombIdToCookieId {
            cookie_id: cookie_id.clone(),
            update_cookie_id: update_cookie_id.clone()
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
        if let Some(err) = result {
            qq_channel
                .send_logger(
                    LogRequest::builder()
                        .level(LogType::Error)
                        .manual()
                        .info("上传七牛云数据源对应最新饼id文件失败".into())
                        .extra(format!(
                            "报错：{err}\n组合id：{comb_id}\n最新饼id：\
                             {cookie_id:#?}\n更新饼id：{update_cookie_id:#?}",
                        ))
                        .build(),
                )
                .await?;
            Err(err)?;
        }
        Ok(())
    }

    /// 删除数据源组合对应最新饼id文件
    pub async fn delete_datasource_comb(
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
        cookie_id: Option<String>, update_cookie_id: Option<String>, comb_id: String,
    ) {
        if Self::delete_datasource_comb(
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
                cookie_id,
                update_cookie_id,
                comb_id,
            )
            .await
            .is_err();
        }
    }

    /// 批量更新数据源组合文件
    pub async fn update_multi_datasource_comb(
        qiniu: QiniuManager, cookie_id: Option<String>, update_cookie_id: Option<String>,
        qq_channel: QqChannelGrpcService, comb_ids: Vec<String>,
    ) {
        let mut handles = Vec::<JoinHandle<()>>::new();
        for comb_id in comb_ids {
            handles.push(tokio::spawn(Self::update_datasource_comb(
                qiniu.clone(),
                qq_channel.clone(),
                cookie_id.clone(),
                update_cookie_id.clone(),
                comb_id,
            )));
        }
        futures::future::join_all(handles).await;
    }
}
