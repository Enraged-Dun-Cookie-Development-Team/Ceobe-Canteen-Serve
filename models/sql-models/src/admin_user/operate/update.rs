use sea_orm::{
    sea_query::IntoCondition, ActiveModelTrait, ColumnTrait, ConnectionTrait,
    DbErr, IntoActiveModel, Set,
};
use sql_connection::database_traits::get_connect::{
    GetDatabaseConnect, GetDatabaseTransaction, TransactionOps,
};

use super::{OperateError, OperateResult, UserSqlOperate};
use crate::admin_user::models::{auth_level::AuthLevel, user};

impl UserSqlOperate {
    pub async fn update_user_name<'db, D>(
        db: &'db D, uid: i32, new_name: String,
    ) -> OperateResult<()>
    where
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        let ctx = db.get_transaction().await?;

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

        let mut user = Self::find_user_by_id_raw(uid, &ctx)
            .await?
            .into_active_model();

        user.username = Set(new_name);

        user.save(&ctx).await?;

        ctx.submit().await?;
        Ok(())
    }

    pub async fn update_user_password<'db, D, Verify, Encode, Map, Err, T>(
        db: &'db D, uid: i32, new_pwd: String, old_pwd: String,
        verify: Verify, encode: Encode, mapping: Map,
    ) -> OperateResult<Result<T, Err>>
    where
        Verify: Fn(&str, &str) -> Result<bool, Err>,
        Encode: Fn(&str) -> Result<String, Err>,
        Map: Fn(user::Model) -> T,
        D: GetDatabaseTransaction<Error = DbErr> + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        let ctx = db.get_transaction().await?;

        let user = Self::find_user_by_id_raw(uid, &ctx).await?;
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
    pub async fn update_user_auth<'db, D>(
        db: &'db D, uid: i32, new_auth: AuthLevel,
    ) -> OperateResult<()>
    where
        D: GetDatabaseConnect<Error = DbErr> + GetDatabaseTransaction + 'db,
        D::Transaction<'db>: ConnectionTrait,
    {
        let db = db.get_transaction().await?;

        let mut user = Self::find_user_by_id_raw(uid, &db)
            .await?
            .into_active_model();

        user.auth = Set(new_auth);

        user.update(&db).await?;

        db.submit().await?;

        Ok(())
    }
}
