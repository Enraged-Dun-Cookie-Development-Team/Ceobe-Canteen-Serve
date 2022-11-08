use std::borrow::Cow;

use checker::{
    prefabs::no_check::NoCheck, CheckExtract, JsonCheckExtract,
    QueryCheckExtract,
};
use crypto_str::Encoder;
use futures::{future, TryFutureExt};
use md5::{Digest, Md5};
use orm_migrate::{
    sql_connection::SqlConnect,
    sql_models::admin_user::operate::UserSqlOperate,
};
use page_size::response::{GenerateListWithPageInfo, ListWithPageInfo};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use resp_result::{resp_try, rtry};

use super::{
    view::{ChangeAuthReq, ChangePassword, DeleteOneUserReq, UserTable},
    PageSizePretreatment, UsernamePretreatment,
};
use crate::{
    middleware::authorize::AuthorizeInfo,
    models::sql::models::auth_level::AuthLevel,
    router::UserAuthBackend,
    serves::backend::user_auth::{
        error::AdminUserError,
        view::{CreateUser, UserInfo, UserName, UserToken},
        AdminUserRResult,
    },
    utils::user_authorize::{AuthInfo, GenerateToken, PasswordEncoder, User},
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
        db: SqlConnect,
        query: QueryCheckExtract<NoCheck<NewUserAuthLevel>, AdminUserError>,
    ) -> AdminUserRResult<CreateUser> {
        resp_try(async {
            let permission = query.0.permission;

            // 生成随机用户名密码
            let rand_username: String = {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            };
            let rand_password: String = {
                thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect()
            };

            let username = rand_username.clone();
            let plaintext_password = rand_password.clone();

            // 进行md5加密
            let rand_password = {
                let mut md5 = Md5::new();
                md5.update(&rand_password);
                let rand_password = md5.finalize();
                let rand_password = hex::encode(rand_password);
                log::debug!(
                    "新建用户密码通过MD5加密后是： {:?}",
                    rand_password
                );
                rand_password
            };

            // 加密密码
            let encode_password = {
                tokio::task::block_in_place(|| {
                    PasswordEncoder::encode(rand_password.into())
                })
            }?;

            // 将用户信息写入数据库
            UserSqlOperate::add_user_with_encoded_password(
                &db,
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

            Ok(user_info)
        })
        .await
    }

    pub async fn login(
        db: SqlConnect,
        CheckExtract(body, _): JsonCheckExtract<
            NoCheck<UserLogin>,
            AdminUserError,
        >,
    ) -> AdminUserRResult<UserToken> {
        resp_try(async {
            let token_info = UserSqlOperate::find_user_and_verify_pwd(
                &db,
                &body.username,
                &body.password,
                |src, dst| {
                    tokio::task::block_in_place(|| {
                        PasswordEncoder::verify(src, &dst)
                    })
                },
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
            Ok(user_token)
        })
        .await
    }

    pub async fn get_info(
        AuthorizeInfo(user): AuthorizeInfo,
    ) -> AdminUserRResult<UserInfo> {
        let AuthInfo { auth, username, .. } = user;

        let user_info = UserInfo {
            roles: [auth],
            name: username,
        };

        Ok(user_info).into()
    }

    pub async fn change_username(
        db: SqlConnect, AuthorizeInfo(user): AuthorizeInfo,
        CheckExtract(username, _): UsernamePretreatment,
    ) -> AdminUserRResult<UserName> {
        resp_try(async {
            let id = user.id;

            let username = username.username;

            UserSqlOperate::update_user_name(&db, id, username.clone())
                .await?;

            Ok(UserName { username })
        })
        .await
    }

    pub async fn change_password(
        db: SqlConnect, AuthorizeInfo(user): AuthorizeInfo,
        CheckExtract(body, _): JsonCheckExtract<
            NoCheck<ChangePassword>,
            AdminUserError,
        >,
    ) -> AdminUserRResult<UserToken> {
        resp_try(async {
            let id = user.id;

            let old_password = body.old_password;
            let new_password = body.new_password;

            let generate_token = UserSqlOperate::update_user_password(
                &db,
                id,
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

            Ok(user_token)
        })
        .await
    }

    // 获取用户列表
    pub async fn user_list(
        db: SqlConnect, CheckExtract(page_size, _): PageSizePretreatment,
    ) -> AdminUserRResult<ListWithPageInfo<UserTable>> {
        resp_try(async {
            // 获取用户列表
            let user_list = UserSqlOperate::find_user_list(&db, page_size)
                .map_ok(|a| {
                    a.into_iter().map(Into::into).collect::<Vec<UserTable>>()
                });
            // 获取用户数量
            let count = UserSqlOperate::get_user_total_number(&db);
            // 异步获取
            let (user_list, count) = future::join(user_list, count).await;

            let resp = user_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    // 修改用户权限
    pub async fn change_auth(
        db: SqlConnect,
        CheckExtract(body, _): JsonCheckExtract<
            NoCheck<ChangeAuthReq>,
            AdminUserError,
        >,
    ) -> AdminUserRResult<()> {
        resp_try(async {
            let ChangeAuthReq { id, auth } = body;
            UserSqlOperate::update_user_auth(&db, id, auth).await?;
            Ok(())
        })
        .await
    }

    // 删除用户
    pub async fn delete_one_user(
        db: SqlConnect,
        CheckExtract(body, _): JsonCheckExtract<
            NoCheck<DeleteOneUserReq>,
            AdminUserError,
        >,
    ) -> AdminUserRResult<()> {
        let uid = body.id;
        rtry!(UserSqlOperate::delete_one_user(&db, uid).await);
        Ok(()).into()
    }
}
