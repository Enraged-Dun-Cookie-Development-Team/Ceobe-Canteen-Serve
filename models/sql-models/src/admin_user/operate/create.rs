use sea_orm::{ActiveModelTrait, ConnectionTrait, DbErr};
use sql_connection::{
    database_traits::get_connect::{
        GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
    },
    sea_orm::Set,
};
use tracing::{info, instrument};

use super::{OperateResult, UserSqlOperate};
use crate::admin_user::models::{auth_level::AuthLevel, user};

impl UserSqlOperate {
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

    #[instrument(skip(db), ret)]
    pub async fn add_user_with_encoded_password<'db, D>(
        db: &'db D, username: String, encoded_pwd: String,
        auth_level: AuthLevel,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + GetDatabaseTransaction + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        info!(
            user.name = username,
            user.password.encoded = encoded_pwd,
            user.auth_level = ?auth_level
        );
        let ctx = db.get_transaction().await?;

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
