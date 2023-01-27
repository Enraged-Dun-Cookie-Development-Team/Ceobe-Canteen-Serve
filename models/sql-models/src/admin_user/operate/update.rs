use std::fmt::Debug;

use sea_orm::{
    sea_query::IntoCondition, ActiveModelTrait, ColumnTrait, ConnectionTrait,
    DbErr, IntoActiveModel, Set,
};
use sql_connection::database_traits::get_connect::{
    GetDatabaseTransaction, TransactionOps,
};
use tracing::{info, instrument};

use super::{OperateError, OperateResult, UserOperate};
use crate::admin_user::models::{auth_level::AuthLevel, user};

impl<'c, C> UserOperate<'c, C>
where
    C: GetDatabaseTransaction<Error = DbErr>,
    C::Transaction<'c>: ConnectionTrait,
{
    #[instrument(ret, skip(self))]
    pub async fn update_user_name(
        &'c self, uid: i32, new_name: String,
    ) -> OperateResult<()> {
        info!(user.id = uid, user.new.name = new_name);
        let ctx = self.get_transaction().await?;

        // check user name exist
        if Self::is_user_exist_raw(
            user::Column::Username.eq(&*new_name).into_condition(),
            &ctx,
        )
        .await?
        {
            return Err(OperateError::ConflictUsername {
                username: new_name,
            });
        }

        let mut user = UserOperate::find_user_by_id_raw(uid, &ctx)
            .await?
            .into_active_model();

        user.username = Set(new_name);

        user.save(&ctx).await?;

        ctx.submit().await?;
        Ok(())
    }

    #[instrument(ret, skip_all)]
    pub async fn update_user_password< Verify, Encode, Map, Err, T>(
        &'c self, uid: i32, new_pwd: String, old_pwd: String,
        verify: Verify, encode: Encode, mapping: Map,
    ) -> OperateResult<Result<T, Err>>
    where
        Verify: Fn(&str, &str) -> Result<bool, Err>,
        Encode: Fn(&str) -> Result<String, Err>,
        Map: Fn(user::Model) -> T,
        T: Debug,
        Err: Debug,
    {
        info!(user.id = uid);
        let ctx = self.get_transaction().await?;

        let user = UserOperate::find_user_by_id_raw(uid, &ctx).await?;
        let pwd_version = user.num_pwd_change;

        // verify password
        let verifying = || {
            // verify old equal to database
            // new pwd != old pwd
            Result::<_, Err>::Ok(
                verify(&user.password, &old_pwd)? && new_pwd != old_pwd,
            )
        };

        // encoding pwd
        let encoded = match verifying() {
            // ok can update
            Ok(true) => encode(&new_pwd),
            Ok(false) => Err(OperateError::PasswordNoChange)?,
            Err(err) => return Ok(Err(err)),
        };

        let mut user = user.into_active_model();

        let resp = match encoded {
            Ok(pwd) => {
                user.password = Set(pwd);
                user.num_pwd_change = Set(pwd_version.wrapping_add(1));
                user.update(&ctx).await.map(mapping)?
            }
            Err(err) => return Ok(Err(err)),
        };

        ctx.submit().await?;
        Ok(Ok(resp))
    }

    // 更新用户权限
    #[instrument(ret, skip(self))]
    pub async fn update_user_auth(
        &'c self, uid: i32, new_auth: AuthLevel,
    ) -> OperateResult<()>
    {
        info!(user.id = uid, user.new.auth_level = ?new_auth);
        let db = self.get_transaction().await?;

        let mut user = UserOperate::find_user_by_id_raw(uid, &db)
            .await?
            .into_active_model();

        user.auth = Set(new_auth);

        user.update(&db).await?;

        db.submit().await?;

        Ok(())
    }
}
