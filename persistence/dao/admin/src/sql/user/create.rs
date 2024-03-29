use db_ops_prelude::{
    database_operates::NoConnect,
    get_connect::{GetDatabaseTransaction, TransactionOps},
    sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr, Set},
    sql_models::admin_user::{self, AuthLevel},
};
use tracing::{info, instrument};

use super::{OperateResult, UserOperate};

impl UserOperate<'_, NoConnect> {
    pub async fn add_user_with_encoded_password_db(
        username: String, encoded_pwd: String, auth_level: AuthLevel,
        db: &impl ConnectionTrait,
    ) -> OperateResult<()> {
        let user_active = admin_user::ActiveModel {
            username: Set(username),
            password: Set(encoded_pwd),
            auth: Set(auth_level),
            ..Default::default()
        };

        user_active.save(db).await?;
        Ok(())
    }
}
impl<'c, C> UserOperate<'c, C>
where
    C: GetDatabaseTransaction<Error = DbErr>,
    C::Transaction<'c>: ConnectionTrait,
{
    #[instrument(skip(self), ret)]
    pub async fn add_with_encoded_password(
        &'c self, username: String, encoded_pwd: String,
        auth_level: AuthLevel,
    ) -> OperateResult<()> {
        info!(
            user.name = username,
            user.password.encoded = encoded_pwd,
            user.auth_level = ?auth_level
        );
        let ctx = self.get_transaction().await?;

        UserOperate::add_user_with_encoded_password_db(
            username,
            encoded_pwd,
            auth_level,
            &ctx,
        )
        .await?;

        ctx.submit().await?;
        Ok(())
    }
}
