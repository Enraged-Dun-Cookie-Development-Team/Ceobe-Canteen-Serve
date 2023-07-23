use db_ops_prelude::{
    database_operates::NoConnect,
    sea_orm::{
        ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter,
        QuerySelect, TransactionTrait,
    },
    sql_models::admin_user::{self, AuthLevel},
};
use tracing::info;

use super::{OperateResult, UserCounts, UserOperate};

impl UserOperate<'_, NoConnect> {
    pub async fn is_user_exist_raw(
        filter: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> OperateResult<bool> {
        let condition = filter.into().unwrap_or_else(Condition::all);
        let resp = admin_user::Entity::find()
            .filter(condition)
            .select_only()
            .column_as(admin_user::Column::Id.count(), "count")
            .into_model::<UserCounts>()
            .one(db)
            .await?
            .unwrap();

        Ok(resp.count != 0)
    }

    pub async fn not_exist_then_create_admin(
        db: &impl TransactionTrait, admin: String, encoded_pwd: String,
    ) -> OperateResult<()> {
        let ctx = db.begin().await?;

        if !Self::is_user_exist_raw(None, &ctx).await? {
            info!(
                user.name = admin,
                user.pwd.encoded = encoded_pwd,
                exist = false
            );

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
