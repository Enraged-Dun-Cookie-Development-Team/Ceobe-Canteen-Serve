use sea_orm::{ActiveModelTrait, ConnectionTrait, TransactionTrait};
use sql_connection::{get_sql_database, sea_orm::Set};

use super::UserSqlOperate;
use crate::admin_user::{
    models::{auth_level::AuthLevel, user},
    AdminUserError,
};

impl UserSqlOperate {
    pub async fn add_user_with_encoded_password_db(
        username: String, encoded_pwd: String, auth_level: AuthLevel,
        db: &impl ConnectionTrait,
    ) -> Result<(), AdminUserError> {
        let user_active = user::ActiveModel {
            username: Set(username),
            password: Set(encoded_pwd),
            auth: Set(auth_level),
            ..Default::default()
        };

        user_active.save(db).await?;
        Ok(())
    }

    pub async fn add_user_with_encoded_password(
        username: String, encoded_pwd: String, auth_level: AuthLevel,
    ) -> Result<(), AdminUserError> {
        let db = get_sql_database();
        let ctx = db.begin().await?;

        Self::add_user_with_encoded_password_db(
            username,
            encoded_pwd,
            auth_level,
            &ctx,
        )
        .await?;

        ctx.commit().await?;
        Ok(())
    }
}