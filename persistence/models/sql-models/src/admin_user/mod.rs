pub use checkers::CheckError;
pub use models::{
    auth_level::AuthLevel,
    user::{ActiveModel, Column, Entity, Model, Relation, UserList},
};

mod checkers;
mod models;

pub mod username {
    pub use super::checkers::username::UsernameChecker as Checker;

    pub type Uncheck = checker::Uncheck<Checker>;
    pub type Checked = checker::Checked<Checker>;
}
