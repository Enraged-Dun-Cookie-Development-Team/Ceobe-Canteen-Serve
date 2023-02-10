

use mongo_models::{mongo_connection::MongoDatabaseOperate, ceobe::user::{operate::ToUserOperate, models::UserChecked, check::user_checker::UserChecker}, mongodb::bson};
use sql_models::{
    fetcher::{
        datasource_config::{
            checkers::FetcherDatasourceConfig,
            models::model_datasource_config::DatasourcePlatform,
            operate::Datasource,
        }, ToFetcherOperate,
    },
    sql_connection::{
        database_traits::get_connect::{
            GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
        },
        sea_orm::{ConnectionTrait, DbErr}, SqlDatabaseOperate,
    },
};
use mongo_models::ceobe::user::check::user_checker::UserUncheck;
use uuid::{Builder, Uuid};
use crate::{view::MobIdReq, error::LogicResult};
use checker::LiteChecker;


pub struct CeobeUserLogic;

impl CeobeUserLogic {
    /// 新建数据源配置
    pub async fn create_user<D>(
        mongo: MongoDatabaseOperate, db: SqlDatabaseOperate, mob_id: MobIdReq,
    ) -> LogicResult<()> 
    where
        D: GetDatabaseConnect + 'static,
        D::Connect: ConnectionTrait
    {
        // TODO: 验证mob_id是否为小刻食堂旗下mob id

        let datasource_uuids = db.fetcher_operate().datasource().find_all_uuid().await?.into_iter().map(|uuid| uuid.into()).collect::<Vec<bson::Uuid>>();

        let userUncheck = UserUncheck::builder()
            .mob_id(mob_id.mob_id)
            .datasource_push(datasource_uuids)
            .build();
        let userChecked: UserChecked =
            UserChecker::lite_check(userUncheck).await?;
        mongo.user().create(userChecked);
        Ok(())
    }
}