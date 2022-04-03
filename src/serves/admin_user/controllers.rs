use actix_web::{get, post, web::Data};
use crypto::digest::Digest;
use crypto_str::Encoder;
use db_entity::sea_orm_active_enums::Auth;
use lazy_static::__Deref;
use orm_migrate::sea_query::Expr;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set,
};

use super::view::ChangePassword;
use crate::{
    database::ServeDatabase,
    generate_controller,
    serves::admin_user::{
        checker::user::{UsernameChecker, UsernameUncheck},
        error::AdminUserError,
        view::{CreateUser, UserInfo, UserName, UserToken},
        AdminUserRResult,
    },
    utils::{
        data_checker::PretreatChecker,
        req_pretreatment::{
            prefabs::{Json, MapErr, Null, Query, ToRResult},
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
    NewUserAuthLevel {
        permission: AuthLevel
    }

    UserLogin {
        username: String
        password: String
    }
}

generate_controller!(
    AdminUserController,
    "/user",
    login,
    create_user,
    get_info,
    change_username,
    change_password
);

#[post("/create")]
async fn create_user(
    auth: AuthenticationLevel<Chef, AdminUserError>,
    query: ReqPretreatment<
        ToRResult<MapErr<Query<NewUserAuthLevel>, AdminUserError>>,
    >,
    db: Data<ServeDatabase>,
) -> AdminUserRResult<CreateUser> {
    let permission = query.0?.permission;

    // token鉴权
    auth.0?;

    // 生成随机用户名密码
    let rand_username: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let rand_password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();
    let username = rand_username.clone();
    let plaintext_password = rand_password.clone();

    // 进行md5加密
    let mut md5 = crypto::md5::Md5::new();
    md5.input_str(&rand_password);
    let rand_password = md5.result_str();
    log::debug!("新建用户密码通过MD5加密后是： {:?}", rand_password);

    // 加密密码
    let encode_password = PasswordEncoder::encode(rand_password.into());
    let encode_password = encode_password.map_err(AdminUserError::from);
    let encode_password = AdminUserRResult::from(encode_password)?;

    // 将用户信息写入数据库
    let user = db_entity::user::ActiveModel {
        username: Set(rand_username),
        password: Set(encode_password.to_string()),
        auth: Set(match permission {
            AuthLevel::Chef => Auth::Chef,
            AuthLevel::Cooker => Auth::Cooker,
            AuthLevel::Architect => Auth::Architect,
        }),
        ..Default::default()
    };
    let user = user
        .save(db.deref().deref())
        .await
        .map_err(AdminUserError::from);
    AdminUserRResult::from(user)?;

    // 返回用户信息
    let user_info = CreateUser {
        username,
        password: plaintext_password,
    };

    Ok(user_info).into()
}

#[post("/login")]
async fn login(
    ReqPretreatment(body): ReqPretreatment<
        ToRResult<MapErr<Json<UserLogin>, AdminUserError>>,
    >,
    db: Data<ServeDatabase>,
) -> AdminUserRResult<UserToken> {
    // 从请求体获取信息
    let body = body?;

    // 查询数据库
    let user = db_entity::user::Entity::find()
        .filter(db_entity::user::Column::Username.eq(body.username))
        .select_only()
        .column(db_entity::user::Column::Password)
        .column(db_entity::user::Column::Id)
        .into_model::<User>()
        .one(db.deref().deref())
        .await
        .map_err(AdminUserError::from);
    // 处理查询结果
    let user = AdminUserRResult::from(user)?;
    let user = user
        .ok_or(UserNotFound)
        .map_err(AuthError::from)
        .map_err(AdminUserError::from);
    let user = AdminUserRResult::from(user)?;

    // 密码转换成crypto_str类型
    let pwd = crypto_str::CryptoString::<PasswordEncoder>::new_raw(
        body.password.clone(),
    );
    let db_password = crypto_str::CryptoString::<PasswordEncoder>::new_crypto(
        user.password,
    );

    // 检查密码是否正确
    let password_correct = pwd
        .verify(&db_password)
        .map_err(AuthError::from)
        .map_err(AdminUserError::from);
    let password_correct = AdminUserRResult::from(password_correct)?;
    if !password_correct {
        AdminUserRResult::<()>::err(AuthError::from(PasswordWrong).into())?;
    }

    // 生成用户token
    let generate_token = User {
        id: user.id,
        password: body.password,
    };
    let token = generate_token.generate().unwrap();

    // 返回用户token
    let user_token = UserToken { token };
    Ok(user_token).into()
}

#[get("/info")]
async fn get_info(
    user: Authentication<AuthError>,
) -> AdminUserRResult<UserInfo> {
    let AuthInfo { auth, username, .. } = user.0?;

    let user_info = UserInfo {
        roles: [auth],
        name: username,
    };

    Ok(user_info).into()
}

#[post("/changeUsername")]
async fn change_username(
    user: Authentication<AuthError>,
    ReqPretreatment(username): ReqPretreatment<
        ToRResult<
            MapErr<
                PretreatChecker<Null, Json<UsernameUncheck>, UsernameChecker>,
                AdminUserError,
            >,
        >,
    >,
    db: Data<ServeDatabase>,
) -> AdminUserRResult<UserName> {
    let id = user.0?.id;

    let username = username?.username;

    let user = db_entity::user::Entity::update_many()
        .col_expr(
            db_entity::user::Column::Username,
            Expr::value(username.clone()),
        )
        .filter(db_entity::user::Column::Id.eq(id))
        .exec(db.deref().deref())
        .await
        .map_err(AdminUserError::from);
    AdminUserRResult::from(user)?;

    let user_name = UserName { username };

    Ok(user_name).into()
}

#[post("/changePassword")]
async fn change_password(
    user: Authentication<AuthError>,
    ReqPretreatment(body): ReqPretreatment<
        ToRResult<MapErr<Json<ChangePassword>, AdminUserError>>,
    >,
    db: Data<ServeDatabase>,
) -> AdminUserRResult<UserToken> {
    let user = user.0?;
    let id = user.id;
    let password = user.password;
    let body = body?;

    let old_password = body.oldpassword;
    let new_password = body.newpassword;

    // 密码转换成crypto_str类型
    let old_password =
        crypto_str::CryptoString::<PasswordEncoder>::new_raw(old_password);
    let password =
        crypto_str::CryptoString::<PasswordEncoder>::new_crypto(password);

    // 检查密码是否正确
    let password_correct = password
        .verify(&old_password)
        .map_err(AuthError::from)
        .map_err(AdminUserError::from);
    let password_correct = AdminUserRResult::from(password_correct)?;
    if !password_correct {
        AdminUserRResult::<()>::err(AuthError::from(PasswordWrong).into())?;
    }

    // 加密密码
    let encode_password =
        PasswordEncoder::encode(new_password.clone().into());
    let encode_password = encode_password.map_err(AdminUserError::from);
    let encode_password = AdminUserRResult::from(encode_password)?;

    // 在数据库修改密码
    let user = db_entity::user::Entity::update_many()
        .col_expr(
            db_entity::user::Column::Password,
            Expr::value(encode_password.to_string()),
        )
        .filter(db_entity::user::Column::Id.eq(id))
        .exec(db.deref().deref())
        .await
        .map_err(AdminUserError::from);
    AdminUserRResult::from(user)?;

    // 生成用户token
    let generate_token = User {
        id,
        password: new_password,
    };
    let token = generate_token.generate().unwrap();

    // 返回用户token
    let user_token = UserToken { token };

    Ok(user_token).into()
}
