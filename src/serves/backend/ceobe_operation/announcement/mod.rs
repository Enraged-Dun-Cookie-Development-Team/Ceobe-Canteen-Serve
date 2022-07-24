use crate::{new_auth_level, utils::user_authorize::{AuthenticationLevel, auth_level::prefabs::{Cooker, Chef}}};

use self::error::CeobeOperationAnnouncementError;

pub mod view;
pub mod error;
pub mod controllers;

new_auth_level! {
    pub AnnouncementAuth => [
        Chef
        Cooker
    ]
}

type AnnouncementAuthentication =
    AuthenticationLevel<AnnouncementAuth, CeobeOperationAnnouncementError>;
