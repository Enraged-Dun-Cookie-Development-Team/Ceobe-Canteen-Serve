use actix_web::{
    get, post,
    web::{self, Data},
};
use crypto_str::Encoder;
use db_entity::sea_orm_active_enums::{self, Auth};
use lazy_static::__Deref;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sea_orm::{ActiveModelTrait, Set};

use crate::{
    database::ServeDatabase,
    generate_controller,
    serves::admin_user::{
        error::AdminUserError, view::CreateUser, AdminUserRResult,
    },
    utils::user_authorize::{AuthLevel, PasswordEncoder, AuthenticationLevel, auth_level::prefabs::Chef},
};

crate::quick_struct! {
    NewUserAuthLevel {
        permission: AuthLevel
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
    auth : AuthenticationLevel<Chef, AdminUserError>,
    web::Query(NewUserAuthLevel { permission }): web::Query<NewUserAuthLevel>,
    db: Data<ServeDatabase>,
) -> AdminUserRResult<CreateUser> {
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
async fn login() -> AdminUserRResult<()> { Ok(()).into() }

#[get("/info")]
async fn get_info() -> AdminUserRResult<()> { Ok(()).into() }

#[post("/changeUsername")]
async fn change_username() -> AdminUserRResult<()> { Ok(()).into() }

#[post("/changePassword")]
async fn change_password() -> AdminUserRResult<()> { Ok(()).into() }
