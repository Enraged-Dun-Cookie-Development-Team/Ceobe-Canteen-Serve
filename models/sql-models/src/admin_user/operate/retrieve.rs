use page_size::{database::OffsetLimit, request::PageSize};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use sql_connection::database_traits::get_connect::{
    GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
};

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

    pub async fn find_user_and_verify_pwd<'db, D, V, M, E, T>(
        db: &'db D, name: &str, pwd: &str, verify: V, mapping: M,
    ) -> OperateResult<Result<T, E>>
    where
        V: Fn(&str, &str) -> Result<bool, E>,
        M: Fn(user::Model) -> T,
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        let ctx = db.get_transaction().await?;

        let user = Self::find_user_by_name_raw(name, &ctx).await?;

        ctx.submit().await?;

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
    pub async fn find_user_with_version_verify<'db, D, M, E, T>(
        db: &'db D, uid: i32, token_version: u32, ok_mapper: M, error: E,
    ) -> OperateResult<Result<T, E>>
    where
        M: Fn(user::Model) -> T,
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;

        let user = Self::find_user_by_id_raw(uid, db).await?;

        if user.num_pwd_change == token_version {
            Ok(Ok(ok_mapper(user)))
        }
        else {
            Ok(Err(error))
        }
    }

    /// 分页获取用户列表
    pub async fn find_user_list<'db, D>(
        db: &'db D, page_size: PageSize,
    ) -> OperateResult<Vec<user::UserList>>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        Ok(user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .column(user::Column::Username)
            .column(user::Column::Auth)
            .offset_limit(page_size)
            .into_model::<user::UserList>()
            .all(db)
            .await?)
    }

    /// 获取用户总数
    pub async fn get_user_total_number<'db, D>(
        db: &'db D,
    ) -> OperateResult<usize>
    where
        D: GetDatabaseConnect<Error = DbErr> + 'db,
        D::Connect<'db>: ConnectionTrait,
    {
        let db = db.get_connect()?;
        user::Entity::find().count(db).await.map_err(Into::into)
    }
}
