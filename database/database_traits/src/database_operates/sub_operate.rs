use super::DatabaseOperate;
use crate::get_connect::GetDatabaseConnect;

pub trait SubOperate<'op>: 'op {
    type Parent;
    fn from_parent(parent: &'op mut Self::Parent) -> Self;
}

pub trait SuperOperate {
    fn child<'r, S: SubOperate<'r, Parent = Self>>(&'r mut self) -> S
    where
        S: 'r;
}

impl<T> SuperOperate for T {
    fn child<'r, S: SubOperate<'r, Parent = Self>>(&'r mut self) -> S
    where
        S: 'r,
    {
        <S as SubOperate>::from_parent(self)
    }
}

pub struct SqlUserOperate<'s, T>(&'s T::Connect<'s>)
where
    T: GetDatabaseConnect + 's;

impl<'s, T> SubOperate<'s> for SqlUserOperate<'s, T>
where
    T: GetDatabaseConnect + 's,
{
    type Parent = DatabaseOperate<T>;

    fn from_parent(parent: &'s mut Self::Parent) -> Self {
        SqlUserOperate(parent.get_connect())
    }
}

fn _sql<C>(mut op: DatabaseOperate<C>)
where
    C: GetDatabaseConnect,
{
    let _v = op.child::<SqlUserOperate<_>>();
}
