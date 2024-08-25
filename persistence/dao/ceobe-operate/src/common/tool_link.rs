use std::ops::Deref;

use db_ops_prelude::database_operates::sub_operate::{
    SubOperate, SuperOperate,
};

use crate::OperationDatabaseOperate;

pub struct ToolLinkOperate<'c, C>(&'c C);

impl<'c, C> Deref for ToolLinkOperate<'c, C> {
    type Target = C;

    fn deref(&self) -> &Self::Target { self.0 }
}

impl<'c, C> SubOperate<'c> for ToolLinkOperate<'c, C> {
    type Parent = OperationDatabaseOperate<'c, C>;

    fn from_parent(parent: &'c Self::Parent) -> Self { Self(parent) }
}

impl<'db, Conn> OperationDatabaseOperate<'db, Conn> {
    pub fn tool_link(&self) -> ToolLinkOperate<'_, Conn> { self.child() }
}
