use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
    QuerySelect, TransactionTrait,
};
use sql_connection::get_sql_database;

use super::{CommonSqlOperate, UserCounts};
use crate::common::{
    sql_models::{auth_level::AuthLevel, user},
    CommonError,
};

impl CommonSqlOperate {
    pub async fn user_exist_raw(
        filter: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<bool, CommonError> {
        let condition = filter.into().unwrap_or_else(|| Condition::all());
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
    ) -> Result<(), CommonError> {
        let db = get_sql_database();
        let ctx = db.begin().await?;

        if Self::user_exist_raw(None, &ctx).await? {
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
