use std::borrow::Cow;

use crypto::digest::Digest;
use crypto_str::Encoder;
use orm_migrate::sql_models::common::operate::CommonSqlOperate;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use request_pretreat::Pretreatment;
use time_usage::sync_time_usage_with_name;

use super::{view::ChangePassword, UsernamePretreatment};
use crate::{
    models::common::sql::sql_models::auth_level::AuthLevel,
    router::UserAuthBackend,
    serves::backend::user_auth::{
        error::AdminUserError,
        view::{CreateUser, UserInfo, UserName, UserToken},
        AdminUserRResult,
    },
    utils::{
        req_pretreatment::{
            prefabs::{Json, MapErr, Query, ToRResult},
            ReqPretreatment,
        },
        user_authorize::{
            auth_level::prefabs::Chef, error::AuthError, AuthInfo,
            Authentication, AuthenticationLevel, GenerateToken,
            PasswordEncoder, User,
        },
    },
};

crate::quick_struct! {
    pub NewUserAuthLevel {
        permission: AuthLevel
    }

    pub UserLogin {
        username: String
        password: String
    }
}

impl UserAuthBackend {
    pub async fn create_user(
        auth: AuthenticationLevel<Chef, AdminUserError>,
        query: ReqPretreatment<
            ToRResult<MapErr<Query<NewUserAuthLevel>, AdminUserError>>,
        >,
    ) -> AdminUserRResult<CreateUser> {
        let permission = query.0.permission;

        // token鉴权
        let _ = auth.0;

        // 生成随机用户名密码
        let rand_username: String =
            sync_time_usage_with_name("生成随机用户名", || {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            });
        let rand_password: String =
            sync_time_usage_with_name("生成随机用户密码", || {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            });

        let username = rand_username.clone();
        let plaintext_password = rand_password.clone();

        // 进行md5加密
        let rand_password =
            sync_time_usage_with_name("随机密码MD5加密", || {
                let mut md5 = crypto::md5::Md5::new();
                md5.input_str(&rand_password);
                let rand_password = md5.result_str();
                log::debug!(
                    "新建用户密码通过MD5加密后是： {:?}",
                    rand_password
                );
                rand_password
            });

        // 加密密码
        let encode_password =
            sync_time_usage_with_name("随机密码加密", || {
                PasswordEncoder::encode(rand_password.into())
            })?;

        // 将用户信息写入数据库
        CommonSqlOperate::add_user_with_encoded_password(
            rand_username,
            encode_password.to_string(),
            permission,
        )
        .await?;

        // 返回用户信息
        let user_info = CreateUser {
            username,
            password: plaintext_password,
        };

        Ok(user_info).into()
    }

    pub async fn login(
        ReqPretreatment(body): ReqPretreatment<
            ToRResult<MapErr<Json<UserLogin>, AdminUserError>>,
        >,
    ) -> AdminUserRResult<UserToken> {
        let token_info = CommonSqlOperate::find_user_and_verify_pwd(
            &body.username,
            &body.password,
            |src, dst| PasswordEncoder::verify(src, &dst),
            |user| {
                User {
                    id: user.id,
                    num_pwd_change: user.num_pwd_change,
                }
            },
        )
        .await??;

        // 生成用户token
        let token = token_info.generate().unwrap();

        // 返回用户token
        let user_token = UserToken { token };
        Ok(user_token).into()
    }

    pub async fn get_info(
        user: Authentication<AuthError>,
    ) -> AdminUserRResult<UserInfo> {
        let AuthInfo { auth, username, .. } = user.0;

        let user_info = UserInfo {
            roles: [auth],
            name: username,
        };

        Ok(user_info).into()
    }

    pub async fn change_username(
        user: Authentication<AuthError>,
        ReqPretreatment(username): UsernamePretreatment,
    ) -> AdminUserRResult<UserName> {
        let id = user.0.id;

        let username = username.username;

        CommonSqlOperate::update_user_name(id as i64, username.clone())
            .await?;

        Ok(UserName { username }).into()
    }

    pub async fn change_password(
        Pretreatment(user): Authentication<AuthError>,
        ReqPretreatment(body): ReqPretreatment<
            ToRResult<MapErr<Json<ChangePassword>, AdminUserError>>,
        >,
    ) -> AdminUserRResult<UserToken> {
        let id = user.id;

        let old_password = body.old_password;
        let new_password = body.new_password;

        let generate_token = CommonSqlOperate::update_user_password(
            id as i64,
            new_password,
            old_password,
            |old, new| PasswordEncoder::verify(old, &new),
            |pwd| {
                PasswordEncoder::encode(Cow::Borrowed(pwd))
                    .map(|pwd| pwd.to_string())
            },
            |user| {
                User {
                    id: user.id,
                    num_pwd_change: user.num_pwd_change,
                }
            },
        )
        .await??;

        let token = generate_token.generate().unwrap();

        // 返回用户token
        let user_token = UserToken { token };

        Ok(user_token).into()
    }
}
