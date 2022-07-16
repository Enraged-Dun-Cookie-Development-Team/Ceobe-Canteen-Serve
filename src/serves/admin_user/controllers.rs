use actix_web::{web::Data};
use crypto::digest::Digest;
use crypto_str::Encoder;
use lazy_static::__Deref;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use request_pretreat::prefabs::DefaultValue;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter,
    Set,
};
use time_usage::{async_time_usage_with_name, sync_time_usage_with_name};

use super::view::ChangePassword;
use crate::{
    router::UserAuthBackend,
    database::ServeDatabase,
    models::common::sql::{auth::Auth, user},
    serves::admin_user::{
        checker::user::{UsernameChecker, UsernameUncheck},
        error::AdminUserError,
        view::{CreateUser, UserInfo, UserName, UserToken},
        AdminUserRResult,
    },
    utils::{
        data_checker::{DataChecker, PretreatChecker},
        req_pretreatment::{
            prefabs::{Json, MapErr, Query, ToRResult},
            ReqPretreatment,
        },
        user_authorize::{
            auth_level::prefabs::Chef,
            error::{AuthError, PasswordWrong, UserNotFound},
            AuthInfo, AuthLevel, Authentication, AuthenticationLevel,
            GenerateToken, PasswordEncoder, User,
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
        db: Data<ServeDatabase>,
    ) -> AdminUserRResult<CreateUser> {
        let permission = query.0.permission;

        // token鉴权
        auth.0;

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
        let user = user::ActiveModel {
            username: Set(rand_username),
            password: Set(encode_password.to_string()),
            auth: Set(match permission {
                AuthLevel::Chef => Auth::Chef,
                AuthLevel::Cooker => Auth::Cooker,
                AuthLevel::Architect => Auth::Architect,
            }),
            ..Default::default()
        };

        async_time_usage_with_name(
            "保存随机生成用户",
            user.save(db.deref().deref()),
        )
        .await
        .map_err(AdminUserError::from)?;

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
        db: Data<ServeDatabase>,
    ) -> AdminUserRResult<UserToken> {
        // 从请求体获取信息
        let body = body;

        // 查询数据库获得user信息
        let user_info = async_time_usage_with_name(
            "查询用户信息",
            user::Entity::find()
                .filter(user::Column::Username.eq(body.username))
                .one(db.deref().deref()),
        )
        .await?
        .ok_or(UserNotFound)
        .map_err(AuthError::from)?;
        let user::Model {
            id,
            password,
            num_pwd_change,
            ..
        } = user_info;

        let password_correct =
            sync_time_usage_with_name("校验密码是否正确", || {
                // 密码转换成crypto_str类型
                let pwd =
                    crypto_str::CryptoString::<PasswordEncoder>::new_raw(
                        body.password.clone(),
                    );
                let db_password =
                    crypto_str::CryptoString::<PasswordEncoder>::new_crypto(
                        password,
                    );

                // 检查密码是否正确
                pwd.verify(&db_password).map_err(AuthError::from)
            })?;

        if !password_correct {
            AdminUserRResult::<()>::err(
                AuthError::from(PasswordWrong).into(),
            )?;
        }

        // 生成用户token
        let generate_token = User { id, num_pwd_change };
        let token = generate_token.generate().unwrap();

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
        ReqPretreatment(username): ReqPretreatment<
            ToRResult<
                MapErr<
                    PretreatChecker<
                        DefaultValue<<UsernameChecker as DataChecker>::Args>,
                        Json<UsernameUncheck>,
                        UsernameChecker,
                    >,
                    AdminUserError,
                >,
            >,
        >,
        db: Data<ServeDatabase>,
    ) -> AdminUserRResult<UserName> {
        let id = user.0.id;

        let username = username.username;

        async_time_usage_with_name(
            "更新用户名称",
            user::Entity::update_many()
                .col_expr(
                    user::Column::Username,
                    Expr::value(username.clone()),
                )
                .filter(user::Column::Id.eq(id))
                .exec(db.deref().deref()),
        )
        .await?;

        let user_name = UserName { username };

        Ok(user_name).into()
    }

    pub async fn change_password(
        user: Authentication<AuthError>,
        ReqPretreatment(body): ReqPretreatment<
            ToRResult<MapErr<Json<ChangePassword>, AdminUserError>>,
        >,
        db: Data<ServeDatabase>,
    ) -> AdminUserRResult<UserToken> {
        let user = user.0;
        let id = user.id;
        let password = user.password;
        let num_pwd_change = user.num_pwd_change;
        let body = body;

        let old_password = body.old_password;
        let new_password = body.new_password;

        // 检查密码是否正确
        let password_correct =
            sync_time_usage_with_name("校验原密码", || {
                // 密码转换成crypto_str类型
                let old_password =
                    crypto_str::CryptoString::<PasswordEncoder>::new_raw(
                        old_password,
                    );
                let password =
                    crypto_str::CryptoString::<PasswordEncoder>::new_crypto(
                        password,
                    );
                password.verify(&old_password).map_err(AuthError::from)
            })?;
        if !password_correct {
            AdminUserRResult::<()>::err(
                AuthError::from(PasswordWrong).into(),
            )?;
        }

        // 加密密码
        let encode_password =
            PasswordEncoder::encode(new_password.clone().into())?;

        // 在数据库修改密码
        async_time_usage_with_name(
            "更新用户密码",
            user::Entity::update_many()
                .col_expr(
                    user::Column::Password,
                    Expr::value(encode_password.to_string()),
                )
                .col_expr(
                    user::Column::NumPwdChange,
                    Expr::value(num_pwd_change + 1),
                )
                .filter(user::Column::Id.eq(id))
                .exec(db.deref().deref()),
        )
        .await?;

        // 生成用户token
        let generate_token = User {
            id,
            num_pwd_change: num_pwd_change + 1,
        };
        let token = generate_token.generate().unwrap();

        // 返回用户token
        let user_token = UserToken { token };

        Ok(user_token).into()
    }
}
