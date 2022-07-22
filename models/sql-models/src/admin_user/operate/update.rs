use sea_orm::{
    sea_query::IntoCondition, ActiveModelTrait, ColumnTrait, IntoActiveModel,
    Set,
};
use sql_connection::get_sql_transaction;

use super::UserSqlOperate;
use crate::admin_user::{models::user, UserError};

impl UserSqlOperate {
    pub async fn update_user_name(
        uid: i64, new_name: String,
    ) -> Result<(), UserError> {
        let ctx = get_sql_transaction().await?;

        // check user name exist
        if Self::is_user_exist_raw(
            user::Column::Username.eq(&*new_name).into_condition(),
            &ctx,
        )
        .await?
        {
            return Err(UserError::ConflictUsername { username: new_name });
        }

        let mut user = Self::find_user_by_id_raw(uid, &ctx)
            .await?
            .into_active_model();

        user.username = Set(new_name);

        user.save(&ctx).await?;

        ctx.commit().await?;
        Ok(())
    }

    pub async fn update_user_password<Verify, Encode, Map, Err, T>(
        uid: i64, new_pwd: String, old_pwd: String, verify: Verify,
        encode: Encode, mapping: Map,
    ) -> Result<Result<T, Err>, UserError>
    where
        Verify: Fn(&str, &str) -> Result<bool, Err>,
        Encode: Fn(&str) -> Result<String, Err>,
        Map: Fn(user::Model) -> T,
    {
        let ctx = get_sql_transaction().await?;

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
            Ok(false) => Err(UserError::PasswordNoChange)?,
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

        ctx.commit().await?;
        Ok(Ok(resp))
    }
}
