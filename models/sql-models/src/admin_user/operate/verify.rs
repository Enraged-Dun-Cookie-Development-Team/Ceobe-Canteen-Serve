use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    QuerySelect, TransactionTrait,
};
use sql_connection::get_sql_database;

use super::{UserCounts, UserSqlOperate, OperateResult};
use crate::admin_user::{
    models::{auth_level::AuthLevel, user}
};

impl UserSqlOperate {
    pub async fn is_user_exist_raw(
        filter: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
        let condition = filter.into().unwrap_or_else(Condition::all);
        let resp = user::Entity::find()
            .filter(condition)
            .select_only()
            .column_as(user::Column::Id.count(), "count")
            .into_model::<UserCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }

    pub async fn not_exist_then_create_admin(
        admin: String, encoded_pwd: String,
    ) -> OperateResult<()> {
        let db = get_sql_database();
        let ctx = db.begin().await?;

        if !Self::is_user_exist_raw(None, &ctx).await? {
            Self::add_user_with_encoded_password_db(
                admin,
                encoded_pwd,
                AuthLevel::Chef,
                &ctx,
            )
            .await?;
        }
        ctx.commit().await?;
        Ok(())
    }
}
