use actix_web::{get, post, web};

use crate::{
    generate_controller,
    serves::admin_user::{
        AdminUserRResult,
    }
};

generate_controller!(
    AdminUserController,
    "/user",
    login,
    create_user,
    get_info,
    change_username,
    change_password
);

#[get("/create")]
async fn create_user() -> AdminUserRResult<()> {
    Ok(()).into()
}

#[post("/login")]
async fn login() -> AdminUserRResult<()> {
    Ok(()).into()
}

#[get("/info")]
async fn get_info() -> AdminUserRResult<()> {
    Ok(()).into()
}

#[post("/changeUsername")]
async fn change_username() -> AdminUserRResult<()> {
    Ok(()).into()
}

#[post("/changePassword")]
async fn change_password() -> AdminUserRResult<()> {
    Ok(()).into()
}