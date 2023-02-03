use std::{fmt::Debug, ops::Deref};

use page_size::{database::WithPagination, request::Paginator};
use sea_orm::{
    sea_query::IntoCondition, ColumnTrait, Condition, ConnectionTrait, DbErr,
    EntityTrait, PaginatorTrait, QueryFilter, QuerySelect,
};
use smallvec::SmallVec;
use sql_connection::database_traits::{
    database_operates::NoConnect,
    get_connect::{
        GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
    },
};
use tap::TapFallible;
use tracing::{info, instrument, Span};

use super::{OperateError, OperateResult, UserOperate};
use crate::admin_user::models::user;

impl UserOperate<'_, NoConnect> {
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
}
impl<'c, C> UserOperate<'c, C>
where
    C: GetDatabaseTransaction<Error = DbErr>,
    C::Transaction<'c>: ConnectionTrait,
{
    #[instrument(ret, skip(self, verify, mapping, pwd))]
    pub async fn find_user_and_verify_pwd<V, M, E, T>(
        &'c self, name: &str, pwd: &str, verify: V, mapping: M,
    ) -> OperateResult<Result<T, E>>
    where
        V: Fn(&str, &str) -> Result<bool, E>,
        M: Fn(user::Model) -> T,
        T: Debug,
        E: Debug,
    {
        info!(user.name = name);
        let ctx = self.get_transaction().await?;

        let user = UserOperate::find_user_by_name_raw(name, &ctx).await?;

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
}

impl<'c, C> UserOperate<'c, C>
where
    C: GetDatabaseConnect,
    C::Connect: ConnectionTrait,
{
    /// 获取并验证密码版本
    #[instrument(ret, skip(self, ok_mapper, error))]
    pub async fn find_user_with_version_verify<M, E, T, OE>(
        &self, uid: i32, token_version: u32, ok_mapper: M, error: OE,
    ) -> OperateResult<Result<T, E>>
    where
        M: Fn(user::Model) -> T,
        OE: Fn() -> E,
        E: Debug,
        T: Debug,
    {
        info!(user.id = uid, user.password.version = token_version);
        let db = self.get_connect();

        let user = UserOperate::find_user_by_id_raw(uid, db).await?;

        if user.num_pwd_change == token_version {
            Ok(Ok(ok_mapper(user)))
        }
        else {
            Ok(Err(error()))
        }
    }

    #[instrument(skip(self))]
    /// 分页获取用户列表
    pub async fn find_user_list(
        &'c self, page_size: Paginator,
    ) -> OperateResult<Vec<user::UserList>> {
        info!(
            userList.page.num = page_size.page.deref(),
            userList.page.size = page_size.size.deref()
        );
        let db = self.get_connect();
        Ok(user::Entity::find()
            .select_only()
            .column(user::Column::Id)
            .column(user::Column::Username)
            .column(user::Column::Auth)
            .with_pagination(page_size)
            .into_model::<user::UserList>()
            .all(db)
            .await?)
        .tap_ok(|list| {
            Span::current()
            .in_scope(||{
                let list = list.iter().map(|user|(&user.username)).collect::<SmallVec<[_;4]>>();
                info!(userList.len = list.len(),  userList.usernames = ?list );
            })
            ;
        })
    }

    #[instrument(skip(self), ret)]
    /// 获取用户总数
    pub async fn get_user_total_number(&'c self) -> OperateResult<u64> {
        let db = self.get_connect();
        user::Entity::find().count(db).await.map_err(Into::into)
    }
}
