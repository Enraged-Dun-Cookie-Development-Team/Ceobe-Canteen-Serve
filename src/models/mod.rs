pub mod mansion;
pub mod admin_user;


crate::generate_model_register!(
    RootModels,
    admin_user::AdminUserModel
);