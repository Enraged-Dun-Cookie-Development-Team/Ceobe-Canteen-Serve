use sea_orm::{
    EntityTrait, FromQueryResult, QuerySelect, Select, SelectModel, Selector,
};

pub trait SelectOnlyModel {
    fn select_cols<E: EntityTrait>(selector: Select<E>) -> Select<E>;

    
}

pub trait SelectPartial {
    type Selector<T>
    where
        T: FromQueryResult;
    fn select_for<M: SelectOnlyModel + FromQueryResult>(
        self,
    ) -> Self::Selector<M>;

    fn select_for_cols<M:SelectOnlyModel>(self)->Self;
}

impl<E: EntityTrait> SelectPartial for Select<E> {
    type  Selector<T> = Selector<SelectModel<T>> where T:FromQueryResult;

    fn select_for<M: SelectOnlyModel + FromQueryResult>(
        self,
    ) -> Self::Selector<M> {
        self.select_for_cols::<M>().into_model::<M>()
    }

    fn select_for_cols<M:SelectOnlyModel>(self)->Self{
        M::select_cols(self.select_only())
    }
}
