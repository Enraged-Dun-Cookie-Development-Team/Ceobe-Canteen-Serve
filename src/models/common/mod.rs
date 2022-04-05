pub mod sql;

crate::generate_model_register!(AdminUserModel, sql::SqlModels);
