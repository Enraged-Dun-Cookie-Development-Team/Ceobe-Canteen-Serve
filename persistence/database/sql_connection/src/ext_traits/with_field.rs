use database_traits::{
    has_field,
    has_scheme::{Field, Has},
};
use sea_orm::{ActiveModelTrait, IntoActiveModel};

use crate::ext_traits::ActiveModelUpdater;

pub struct WithField<F: Field, A: Has<F>, T>(pub A::Ty, pub T);

impl<F, A, T> IntoActiveModel<A> for WithField<F, A, T>
where
    F: Field,
    A: Has<F> + ActiveModelTrait,
    T: IntoActiveModel<A>,
{
    fn into_active_model(self) -> A {
        let mut this = self.1.into_active_model();
        Has::set(&mut this, self.0);
        this
    }
}

impl<F, A, T> ActiveModelUpdater<A> for WithField<F, A, T>
where
    A: ActiveModelTrait + Has<F>,
    T: ActiveModelUpdater<A>,
    F: Field,
{
    fn update_active(self, active_model: &mut A) {
        Has::<F>::set(active_model, self.0);
        self.1.update_active(active_model);
    }
}

pub trait With<A>: Sized {
    fn with<F>(self, _: F, field: A::Ty) -> WithField<F, A, Self>
    where
        F: Field,
        A: Has<F>,
    {
        WithField(field, self)
    }
}

impl<T: Sized, A> With<A> for T {}

has_field!(FieldOrder:order);
pub type UpdateWithOrder<A, T> = WithField<FieldOrder, A, T>;
