pub trait SubOperate<'p, 's>: 's
where
    'p: 's,
{
    type Parent<'parent>: 'parent
    where
        'parent: 's;
    fn from_parent<'parent>(parent: &'p Self::Parent<'parent>) -> Self
    where
        'parent: 's;
}

pub trait SubMutOperate<'op>: 'op {
    type Parent;
    fn from_mut_parent(parent: &'op mut Self::Parent) -> Self;
}

pub trait SuperOperate {
    fn child<'this, 'r, 's, S>(&'r self) -> S
    where
        S: SubOperate<'r, 's, Parent<'this> = Self>,
        Self: 'r,
        'r: 's,
        'this: 's,
    {
        <S as SubOperate>::from_parent(self)
    }
}

impl<T> SuperOperate for T {}

pub trait SuperMutOperate {
    fn mut_child<'r, S>(&'r mut self) -> S
    where
        S: 'r,
        S: SubMutOperate<'r, Parent = Self>,
    {
        <S as SubMutOperate>::from_mut_parent(self)
    }
}

impl<T> SuperMutOperate for T {}
