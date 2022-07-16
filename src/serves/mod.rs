mod admin_user;
mod ceobe_push;
pub mod mansion;

pub mod admin_group {
    pub use super::admin_user::AdminUserController;

    crate::generate_controller!(
        AdminWrapController,
        "/admin",
        AdminUserController
    );
}
pub mod non_admin_group {

    pub use super::ceobe_push::CeobeController;

    crate::generate_controller!(
        CanteenWrapController,
        "/canteen"
    );
}
