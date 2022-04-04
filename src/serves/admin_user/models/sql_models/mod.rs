use crate::database::model_register::SqlModelRegister;

pub mod auth;
pub mod user;

pub(super) fn sql_register(register: SqlModelRegister) -> SqlModelRegister {
    register.register_model(user::Entity)
}
