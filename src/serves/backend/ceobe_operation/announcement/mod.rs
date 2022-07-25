use self::error::CeobeOperationAnnouncementError;
use crate::{
    new_auth_level,
    utils::user_authorize::{
        auth_level::prefabs::{Chef, Cooker},
        AuthenticationLevel,
    },
};

pub mod controllers;
pub mod error;
pub mod view;

new_auth_level! {
    pub AnnouncementAuth => [
        Chef
        Cooker
    ]
}

type AnnouncementAuthentication =
    AuthenticationLevel<AnnouncementAuth, CeobeOperationAnnouncementError>;
