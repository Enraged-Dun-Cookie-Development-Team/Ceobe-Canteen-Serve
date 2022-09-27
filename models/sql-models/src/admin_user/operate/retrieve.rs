use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait,
    EntityTrait, QueryFilter, QuerySelect, PaginatorTrait,
};
use sql_connection::{get_sql_database, get_sql_transaction};

use super::{OperateError, OperateResult, UserSqlOperate};
use crate::admin_user::models::user;

impl UserSqlOperate {
    pub async fn query_one_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> OperateResult<user::Model> {
        let condition = condition.into().unwrap_or_else(Condition::all);

        user::Entity::find()
            .filter(condition)
            .one(db)
            .await?
            .ok_or(OperateError::UserNotExist)
    }

    pub async fn query_all_user_raw(
        condition: impl Into<Option<Condition>>, db: &impl ConnectionTrait,
    ) -> OperateResult<Vec<user::Model>> {
        Ok(user::Entity::find()
            .filter(condition.into().unwrap_or_else(Condition::all))
            .all(db)
            .await?)
    }

    pub async fn find_user_by_name_raw(
        username: &str, db: &impl ConnectionTrait,
    ) -> OperateResult<user::Model> {
        Self::query_one_user_raw(
            user::Column::Username.eq(username).into_condition(),
            db,
        )
        .await
    }

    pub async fn find_user_by_id_raw(
        uid: i32, db: &impl ConnectionTrait,
    ) -> OperateResult<user::Model> {
        Self::query_one_user_raw(
            user::Column::Id.eq(uid).into_condition(),
            db,
        )
        .await
    }

    // 获取并验证密码
    pub async fn find_user_and_verify_pwd<V, M, E, T>(
        name: &str, pwd: &str, verify: V, mapping: M,
    ) -> OperateResult<Result<T, E>>
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
            Ok(false) => Err(OperateError::PasswordWrong),
            Err(err) => Ok(Err(err)),
        }
    }

    /// 获取并验证密码版本
    pub async fn find_user_with_version_verify<M, E, T>(
        uid: i32, token_version: u32, ok_mapper: M, error: E,
    ) -> OperateResult<Result<T, E>>
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

    /// 分页获取用户列表
    pub async fn find_user_list(page:u64, size:u64) -> OperateResult<Vec<user::UserList>> {
        let db = get_sql_database();
        Ok(user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .column(user::Column::Username)
            .column(user::Column::Auth)
            .offset((page-1)*size).limit(size)
            .into_model::<user::UserList>()
            .all(db)
            .await?)
    }

    /// 获取用户总数
    pub async fn get_user_total_number() -> OperateResult<u64> {
        let db = get_sql_database();
        Ok(user::Entity::find()
            .count(db)
            .await?.try_into().unwrap())
    }
}
