use crate::utils::mvc_utils::as_sql_register;

pub mod sql_models;

crate::generate_model_register!(
    AdminUserModel,
    as_sql_register(sql_models::sql_register)
);
