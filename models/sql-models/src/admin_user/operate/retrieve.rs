use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait,
    EntityTrait, QueryFilter,
};
use sql_connection::{get_sql_database, get_sql_transaction};

use super::UserSqlOperate;
use crate::admin_user::{models::user, AdminUserError};

impl UserSqlOperate {
    pub async fn query_one_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<user::Model, AdminUserError> {
        let condition = condition.into().unwrap_or_else(Condition::all);

        user::Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or(AdminUserError::UserNotExist)
    }

    pub async fn query_all_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<Vec<user::Model>, AdminUserError> {
        Ok(user::Entity::find()
            .filter(condition.into().unwrap_or_else(Condition::all))
            .all(db)
            .await?)
    }

    pub async fn find_user_by_name_raw(
        username: &str, db: &impl ConnectionTrait,
    ) -> Result<user::Model, AdminUserError> {
        Self::query_one_user_raw(
            user::Column::Username.eq(username).into_condition(),
            db,
        )
        .await
    }

    pub async fn find_user_by_id_raw(
        uid: i64, db: &impl ConnectionTrait,
    ) -> Result<user::Model, AdminUserError> {
        Self::query_one_user_raw(
            user::Column::Id.eq(uid).into_condition(),
            db,
        )
        .await
    }

    pub async fn find_user_and_verify_pwd<V, M, E, T>(
        name: &str, pwd: &str, verify: V, mapping: M,
    ) -> Result<Result<T, E>, AdminUserError>
    where
        V: Fn(&str, &str) -> Result<bool, E>,
        M: Fn(user::Model) -> T,
    {
        let ctx = get_sql_transaction().await?;

        let user = Self::find_user_by_name_raw(name, &ctx).await?;

        match verify(&user.password, pwd) {
            Ok(true) => {
                let resp = mapping(user);
                Ok(Ok(resp))
            }
            Ok(false) => Err(AdminUserError::PasswordWrong),
            Err(err) => Ok(Err(err)),
        }
    }

    pub async fn find_user_with_version_verify<M, E, T>(
        uid: i64, token_version: u32, ok_mapper: M, error: E,
    ) -> Result<Result<T, E>, AdminUserError>
    where
        M: Fn(user::Model) -> T,
    {
        let db = get_sql_database();

        let user = Self::find_user_by_id_raw(uid, db).await?;

        if user.num_pwd_change == token_version {
            Ok(Ok(ok_mapper(user)))
        }
        else {
            Ok(Err(error))
        }
    }
}
