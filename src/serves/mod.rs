mod admin_user;
mod ceobe_push;
mod mansion;
#[cfg(test)] mod mock_mongo;

pub mod admin_group {
    pub use super::{
        admin_user::AdminUserController, mansion::MansionController,
    };

    crate::generate_controller!(
        AdminWrapController,
        "/admin",
        AdminUserController,
        MansionController
    );
}
pub mod non_admin_group {

    pub use super::ceobe_push::CeobeController;
    pub use super::mansion::MansionFrontController;

    crate::generate_controller!(CanteenWrapController, "/canteen", MansionFrontController);
}
