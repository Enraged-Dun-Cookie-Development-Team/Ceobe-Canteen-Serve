use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait,
    EntityTrait, QueryFilter,
};
use sql_connection::get_sql_transaction;

use super::CommonSqlOperate;
use crate::common::{sql_models::user, CommonError};

impl CommonSqlOperate {
    pub async fn query_one_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<user::Model, CommonError> {
        let condition = condition.into().unwrap_or_else(Condition::all);

        Ok(user::Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or(CommonError::UserNotExist)?)
    }

    pub async fn query_all_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> Result<Vec<user::Model>, CommonError> {
        Ok(user::Entity::find()
            .filter(condition.into().unwrap_or_else(Condition::all))
            .all(db)
            .await?)
    }

    pub async fn find_user_by_name_raw(
        username: &str, db: &impl ConnectionTrait,
    ) -> Result<user::Model, CommonError> {
        Self::query_one_user_raw(
            user::Column::Username.eq(username).into_condition(),
            db,
        )
        .await
    }

    pub async fn find_user_by_id_raw(
        uid: i64, db: &impl ConnectionTrait,
    ) -> Result<user::Model, CommonError> {
        Self::query_one_user_raw(
            user::Column::Id.eq(uid).into_condition(),
            db,
        )
        .await
    }

    pub async fn find_user_and_verify_pwd<V, M, E, T>(
        name: &str, pwd: &str, verify: V, mapping: M,
    ) -> Result<Result<T, E>, CommonError>
    where
        V: Fn(&str, &str) -> Result<bool, E>,
        M: Fn(user::Model) -> T,
    {
        let ctx = get_sql_transaction().await?;

        let user = Self::find_user_by_name_raw(&name, &ctx).await?;

        match verify(&user.password, &pwd) {
            Ok(true) => {
                let resp = mapping(user);
                Ok(Ok(resp))
            }
            Ok(false) => Err(CommonError::PasswordWrong),
            Err(err) => Ok(Err(err)),
        }
    }
}
