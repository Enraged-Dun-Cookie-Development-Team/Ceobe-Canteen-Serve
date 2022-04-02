use actix_web::{
    get, post,
    web::{self, Data},
};
use crypto_str::Encoder;
use db_entity::sea_orm_active_enums::{self, Auth};
use lazy_static::__Deref;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QuerySelect, Set,
};

use crate::{
    database::ServeDatabase,
    generate_controller,
    serves::admin_user::{
        error::AdminUserError,
        view::{CreateUser, UserToken},
        AdminUserRResult,
    },
    utils::{
        req_pretreatment::{
            prefabs::{Json, MapErr, Query, ToRResult},
            ReqPretreatment,
        },
        user_authorize::{
            auth_level::prefabs::Chef,
            error::{AuthError, PasswordWrong, UserNotFound},
            AuthLevel, AuthenticationLevel, GenerateToken, PasswordEncoder,
            User,
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
async fn get_info() -> AdminUserRResult<()> { Ok(()).into() }

#[post("/changeUsername")]
async fn change_username() -> AdminUserRResult<()> { Ok(()).into() }

#[post("/changePassword")]
async fn change_password() -> AdminUserRResult<()> { Ok(()).into() }
