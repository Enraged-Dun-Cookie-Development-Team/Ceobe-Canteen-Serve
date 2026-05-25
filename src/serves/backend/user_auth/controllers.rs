use std::borrow::Cow;

use authorize_server::{
    AuthorizedUser, JwtTokenConv, admin::AuthorizedAdminUser,
};
use axum::{Json, extract::Query};
use axum_resp_result::{MapReject, resp_result, resp_try};
use checker::CheckExtract;
use crypto_str::Encoder;
use futures::{TryFutureExt, future};
use md5::{Digest, Md5};
use page_size::response::{GenerateListWithPageInfo, ListWithPageInfo};
use persistence::{
    admin::{ToAdmin, models::AuthLevel, user::ToUser},
    mysql::SqlDatabaseOperate,
};
use rand::{Rng, distributions::Alphanumeric, thread_rng};
use tracing::{debug, instrument};
use tracing_unwrap::ResultExt;

use super::{
    PageSizePretreatment, UsernamePretreatment,
    error::AdminUserError,
    view::{ChangeAuthReq, ChangePassword, DeleteOneUserReq, UserTable},
};
use crate::{
    middleware::authorize::AuthorizeInfo,
    router::UserAuthBackend,
    serves::backend::user_auth::{
        AdminUserRResult,
        error::SelfDeleteError,
        view::{CreateUser, UserInfo, UserName, UserToken},
    },
    utils::user_authorize::{AuthInfo, PasswordEncoder, User},
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
    #[instrument(ret, skip(db, permission))]
    pub async fn create_user(
        db: SqlDatabaseOperate,
        MapReject(NewUserAuthLevel { permission }): MapReject<
            Query<NewUserAuthLevel>,
            AdminUserError,
        >,
    ) -> AdminUserRResult<CreateUser> {
        resp_try(async {
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

                debug!(newUser.password.md5 = rand_password);
                rand_password
            };

            // 加密密码
            let encode_password = {
                tokio::task::block_in_place(|| {
                    PasswordEncoder::encode(rand_password.into())
                })
            }?;

            // 将用户信息写入数据库
            db.admin()
                .user()
                .add_with_encoded_password(
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

    #[instrument(ret, skip_all)]
    pub async fn login(
        db: SqlDatabaseOperate,
        MapReject(UserLogin {
            ref username,
            ref password,
        }): MapReject<Json<UserLogin>, AdminUserError>,
    ) -> AdminUserRResult<UserToken> {
        resp_try(async {
            let token_info = db
                .admin()
                .user()
                .find_user_and_verify_pwd(
                    username,
                    password,
                    |src, dst| {
                        tokio::task::block_in_place(|| {
                            PasswordEncoder::verify(src, &dst)
                        })
                    },
                    User::from_model,
                )
                .await??;

            // 生成用户token
            let token = token_info
                .to_jwt_token()
                .expect_or_log("Conv To JWT Token Error");

            // 返回用户token
            let user_token = UserToken { token };
            Ok(user_token)
        })
        .await
    }

    #[instrument(ret, skip_all)]
    pub async fn get_info(
        AuthorizedUser(user): AuthorizedAdminUser,
    ) -> AdminUserRResult<UserInfo> {
        let AuthInfo { auth, username, .. } = user;

        let user_info = UserInfo {
            roles: [auth],
            name: username,
        };

        Ok(user_info).into()
    }

    #[instrument(ret, skip(db, user))]
    pub async fn change_username(
        db: SqlDatabaseOperate, AuthorizedUser(user): AuthorizeInfo,
        CheckExtract(username): UsernamePretreatment,
    ) -> AdminUserRResult<UserName> {
        resp_try(async {
            let id = user.id;

            let username = username.username;
            db.admin()
                .user()
                .update_user_name(id, username.clone())
                .await?;

            Ok(UserName { username })
        })
        .await
    }

    #[instrument(ret, skip(db, user))]
    pub async fn change_password(
        db: SqlDatabaseOperate, AuthorizedUser(user): AuthorizeInfo,
        MapReject(ChangePassword {
            new_password,
            old_password,
        }): MapReject<Json<ChangePassword>, AdminUserError>,
    ) -> AdminUserRResult<UserToken> {
        resp_try(async {
            let id = user.id;

            let generate_token = db
                .admin()
                .user()
                .update_user_password(
                    id,
                    new_password,
                    old_password,
                    |old, new| PasswordEncoder::verify(old, &new),
                    |pwd| {
                        PasswordEncoder::encode(Cow::Borrowed(pwd))
                            .map(|pwd| pwd.to_string())
                    },
                    User::from_model,
                )
                .await??;

            let token = generate_token
                .to_jwt_token()
                .expect_or_log("Conv to JWT Failure");

            // 返回用户token
            let user_token = UserToken { token };

            Ok(user_token)
        })
        .await
    }

    #[instrument(ret, skip(db))]
    // 获取用户列表
    pub async fn user_list(
        db: SqlDatabaseOperate, CheckExtract(page_size): PageSizePretreatment,
    ) -> AdminUserRResult<ListWithPageInfo<UserTable>> {
        resp_try(async {
            // 异步获取用户列&用户数量
            let (user_list, count) = future::join(
                db.admin().user().find_user_list(page_size).map_ok(|a| {
                    a.into_iter().map(Into::into).collect::<Vec<UserTable>>()
                }),
                db.admin().user().get_user_total_number(),
            )
            .await;

            let resp = user_list?.with_page_info(page_size, count?);

            Ok(resp)
        })
        .await
    }

    #[instrument(ret, skip(db))]
    // 修改用户权限
    pub async fn change_auth(
        db: SqlDatabaseOperate,
        MapReject(body): MapReject<Json<ChangeAuthReq>, AdminUserError>,
    ) -> AdminUserRResult<()> {
        resp_try(async {
            let ChangeAuthReq { id, auth } = body;
            db.admin().user().update_user_auth(id, auth).await?;
            Ok(())
        })
        .await
    }

    /// 删除用户
    #[instrument(ret, skip(db))]
    #[resp_result]
    pub async fn delete_one_user(
        db: SqlDatabaseOperate, AuthorizedUser(user): AuthorizedAdminUser,
        MapReject(DeleteOneUserReq { id }): MapReject<
            Json<DeleteOneUserReq>,
            AdminUserError,
        >,
    ) -> Result<(), AdminUserError> {
        if user.id == id {
            Err(SelfDeleteError.into())
        }
        else {
            db.admin().user().delete_one(id).await?;
            Ok(())
        }
    }
}
