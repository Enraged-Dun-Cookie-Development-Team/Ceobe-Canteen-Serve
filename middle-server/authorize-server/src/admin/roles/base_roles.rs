use super::{AuthLevel, UserRoleVerify};

macro_rules! base_role_gen {
    ($id:ident) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $id;

        impl UserRoleVerify for $id {
            const ROLE_NAME: &'static str = stringify!($id);

            fn access_verify(level: &AuthLevel) -> bool {
                matches!(level, &AuthLevel::$id)
            }
        }
    };
}

base_role_gen!(Chef);
base_role_gen!(Cooker);
base_role_gen!(Architect);
base_role_gen!(Porter);

/// anyone can access
#[derive(Clone, Copy, Debug)]
pub struct Any;

impl UserRoleVerify for Any {
    const ROLE_NAME: &'static str = "Any";

    fn access_verify(_: &AuthLevel) -> bool { true }
}

/// no one can access
#[derive(Clone, Copy, Debug)]
pub struct Nil;

impl UserRoleVerify for Nil {
    const ROLE_NAME: &'static str = "Nil";

    fn access_verify(_: &AuthLevel) -> bool { false }
}
