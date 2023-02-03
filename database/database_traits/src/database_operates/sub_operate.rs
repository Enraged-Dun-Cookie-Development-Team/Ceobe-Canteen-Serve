pub trait SubOperate<'s>: 's {
    type Parent;
    fn from_parent(parent: &'s Self::Parent) -> Self;
}

pub trait SubMutOperate<'op>: 'op {
    type Parent;
    fn from_mut_parent(parent: &'op mut Self::Parent) -> Self;
}

pub trait SuperOperate {
    fn child<'s, S>(&'s self) -> S
    where
        S: SubOperate<'s, Parent = Self>,
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
