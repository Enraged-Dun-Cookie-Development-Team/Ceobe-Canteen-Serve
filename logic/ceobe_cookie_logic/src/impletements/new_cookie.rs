use ceobe_cookie::ToCeobe;
use ceobe_qiniu_upload::QiniuManager;
use ceobe_user::ToCeobeUser;
use db_ops_prelude::{get_connect::GetDatabaseConnect, SqlDatabaseOperate};
use fetcher::{
    datasource_combination::{
        DatasourceCombinationOperate, ToDatasourceCombination,
    },
    datasource_config::DatasourceOperate,
    ToFetcher,
};
use general_request_client::client::RequestClient;
use mob_push_server::{mob_push, PushManager};
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use qiniu_cdn_upload::upload;

use crate::{
    error::LogicResult,
    impletements::CeobeCookieLogic,
    view::{
        CombIdToCookieId, CombIdToCookieIdPlayLoad, DeleteObjectName,
        NewCookieReq, PushInfo,
    },
};

impl CeobeCookieLogic {
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate,
        request_client: RequestClient, mut mob: PushManager,
        qiniu: QiniuManager, new_cookie: NewCookieReq,
    ) -> LogicResult<()> {
        let db = sql.get_connect();
        // 查询数据源相关信息
        let datasource_info =
            DatasourceOperate::find_model_by_datasource_and_unique_key(
                db,
                &new_cookie.source.datasource,
                &new_cookie.source.unique,
            )
            .await?;

        // 查询用户列表
        let user_list = mongo
            .ceobe()
            .user()
            .property()
            .find_user_list_by_datasource(datasource_info.unique_id.into())
            .await?;

        // 获取新饼需要改变的数据源组合
        let comb_ids =
            DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
                db,
                datasource_info.id,
            )
            .await?;
        // 更新最新饼id对象储存
        // 删除对象储存中的数据源组合文件
        for comb_id in comb_ids {
            let err = qiniu
                .delete(DeleteObjectName {
                    file_name: comb_id.clone(),
                })
                .await
                .err();
            if err.is_some() {
                // TODO: qq频道告警
            }

            let source = CombIdToCookieId {
                cookie_id: Some(new_cookie.cookie_id),
            };
            let payload = CombIdToCookieIdPlayLoad {
                file_name: &comb_id,
            };

            // 上传数据源组合到对象储存[重试3次]
            let mut result = Option::<ceobe_qiniu_upload::Error>::None;
            for _ in 0..3 {
                result = upload(&qiniu, &source, payload).await.err();
                if result.is_none() {
                    break;
                }
            }
            if let Some(err) = result {
                // TODO: qq频道告警
            }
        }

        // mob推送新饼
        let content = PushInfo {
            content: new_cookie.content.text,
            datasource_name: datasource_info.nickname,
            image_url: new_cookie.content.image_url,
            icon_url: datasource_info.avatar,
        };
        if mob_push::<'_, _, String, _>(&mut mob, &content, &user_list)
            .await
            .is_err()
        {
            // TODO: qq频道告警
        }

        Ok(())
    }
}
