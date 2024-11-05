use std::ops::{Deref, DerefMut};
mod dao_macros;
mod db_op_impls;
pub mod operate_trait;
pub mod sub_operate;

#[derive(Clone)]
pub struct DatabaseOperate<C> {
    connect: C,
}

impl<C> DatabaseOperate<C> {
    pub fn new(connect: C) -> Self { Self { connect } }
}

impl<C> DerefMut for DatabaseOperate<C> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.connect }
}

impl<C> Deref for DatabaseOperate<C> {
    type Target = C;

    fn deref(&self) -> &Self::Target { &self.connect }
}

pub struct NoConnect;
