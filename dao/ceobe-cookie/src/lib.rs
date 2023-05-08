use std::ops::Deref;

use abstract_database::ceobe::CeobeDatabaseOperate;
pub use abstract_database::ceobe::ToCeobe;
use db_ops_prelude::database_operates::sub_operate::{
    SubOperate, SuperOperate,
};
pub use db_ops_prelude::mongo_models::ceobe::cookie::analyze::models::meta::Timestamp as CookieTimestamp;

#[path = "mongo/analyze/mod.rs"] pub mod analyze;
#[path = "mongo/temp_list/mod.rs"] pub mod temp_list;

pub struct CookieDatabaseOperate<'db, Conn>(&'db Conn);

impl<'db, Conn> Deref for CookieDatabaseOperate<'db, Conn> {
    type Target = Conn;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'db, Conn> SubOperate<'db> for CookieDatabaseOperate<'db, Conn> {
    type Parent = CeobeDatabaseOperate<'db, Conn>;

    fn from_parent(parent: &'db Self::Parent) -> Self { Self(parent) }
}

pub trait ToCookie<C> {
    fn cookie(&self) -> CookieDatabaseOperate<'_, C>;
}

impl<C> ToCookie<C> for CeobeDatabaseOperate<'_, C> {
    fn cookie(&self) -> CookieDatabaseOperate<'_, C> { self.child() }
}
