use ceobe_cookie::ToCeobe;
use ceobe_qiniu_upload::QiniuManager;
use ceobe_user::ToCeobeUser;
use db_ops_prelude::{get_connect::GetDatabaseConnect, SqlDatabaseOperate};
use fetcher::{
    datasource_combination::DatasourceCombinationOperate,
    datasource_config::DatasourceOperate,
};
use futures::future;
use mob_push_server::PushManager;
use mongo_migration::mongo_connection::MongoDatabaseOperate;
use qiniu_service::QiniuService;

use crate::{
    error::LogicResult,
    impletements::CeobeCookieLogic,
    view::{NewCookieReq, PushInfo},
};

impl CeobeCookieLogic {
    pub async fn new_cookie(
        mongo: MongoDatabaseOperate, sql: SqlDatabaseOperate,
        mut mob: PushManager, qiniu: QiniuManager, new_cookie: NewCookieReq,
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

        let (datasource_error, qiniu_err) = future::join(
            async {
                // 查询用户列表
                let result = mongo
                    .ceobe()
                    .user()
                    .property()
                    .find_user_list_by_datasource(datasource_info.unique_id.into())
                    .await;
                match result {
                    Ok(user_list) => {
                        // mob推送新饼
                        let content = PushInfo::builder()
                        .content(new_cookie.content.text)
                        .datasource_name(datasource_info.nickname)
                        .image_url(new_cookie.content.image_url)
                        .icon_url(datasource_info.avatar)
                        .build();

                        if mob.mob_push::<_, String, _>(&content, &user_list)
                            .await
                            .is_err()
                        {
                            // TODO: qq频道告警
                        }

                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            },
            async {
                // 获取新饼需要改变的数据源组合
                let result =
                DatasourceCombinationOperate::find_comb_id_by_one_datasource_raw(
                    db,
                    datasource_info.id,
                )
                .await;
                match result {
                    Ok(comb_ids) => {
                        // 更新最新饼id对象储存
                        // 删除对象储存中的数据源组合文件
                        QiniuService::update_multi_datasource_comb(&qiniu, Some(new_cookie.cookie_id.to_string()), comb_ids).await;
                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            }
        )
        .await;

        datasource_error?;
        qiniu_err?;

        Ok(())
    }
}
