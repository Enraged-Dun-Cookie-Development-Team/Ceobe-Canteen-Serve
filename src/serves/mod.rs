mod admin_user;
mod ceobe_push;
mod mansion;
#[cfg(test)] mod mock_mongo;

pub mod admin_group {
    pub use super::{
        admin_user::AdminUserController,
        mansion::{MansionController, MansionModel},
    };

    crate::generate_controller!(
        AdminWrapController,
        "/admin",
        AdminUserController,
        MansionController
    );

    crate::generate_model_register!(
        AdminWrapModel,
        MansionModel
    );
}
pub mod non_admin_group {

    pub use super::ceobe_push::CeobeController;

    crate::generate_controller!(CanteenWrapController, "/canteen");

    crate::generate_model_register!(
        CanteenWrapModel
    );
}
