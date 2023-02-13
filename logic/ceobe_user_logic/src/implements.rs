

use mongo_models::{mongo_connection::MongoDatabaseOperate, ceobe::user::{operate::ToUserOperate, models::UserChecked, check::user_checker::UserChecker}, mongodb::bson};
use sql_models::{
    fetcher::
        ToFetcherOperate,
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr}, SqlDatabaseOperate,
    },
};
use mongo_models::ceobe::user::check::user_checker::UserUncheck;
use crate::{view::MobIdReq, error::LogicResult};
use checker::LiteChecker;


pub struct CeobeUserLogic;

impl CeobeUserLogic {
    /// 新建数据源配置
    pub async fn create_user(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: MobIdReq,
    ) -> LogicResult<()> {
        // TODO: 验证mob_id是否为小刻食堂旗下mob id


        // 获取所有数据源的uuid列表
        let datasource_uuids = db.fetcher_operate().datasource().find_all_uuid().await?.into_iter().map(|uuid| uuid.into()).collect::<Vec<bson::uuid::Uuid>>();

        // 拼接数据
        let user_uncheck = UserUncheck::builder()
            .mob_id(mob_id.mob_id)
            .datasource_push(datasource_uuids)
            .build();
        // 验证数据
        let user_checked: UserChecked =
            UserChecker::lite_check(user_uncheck).await?;
        // 将用户信息存入数据库
        mongo.user().create(user_checked).await?;
        Ok(())
    }
}