use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};
use sql_connection::{
    database_traits::get_connect::{GetDatabaseTransaction, TransactionOps},
    sea_orm::Set,
};
use tracing::{info, instrument};

use super::{OperateResult, UserOperate};
use crate::admin_user::models::{auth_level::AuthLevel, user};

impl<'c, C> UserOperate<'c, C> {
    pub async fn add_user_with_encoded_password_db(
        username: String, encoded_pwd: String, auth_level: AuthLevel,
        db: &impl ConnectionTrait,
    ) -> OperateResult<()> {
        let user_active = user::ActiveModel {
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

        Self::add_user_with_encoded_password_db(
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
