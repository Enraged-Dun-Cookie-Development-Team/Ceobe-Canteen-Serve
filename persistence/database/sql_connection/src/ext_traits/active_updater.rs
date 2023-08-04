use sea_orm::{ActiveModelTrait, IntoActiveModel};
use database_traits::has_field;
use database_traits::has_scheme::{Field, Has};

pub trait ActiveModelUpdater<A: ActiveModelTrait> {
    fn update_active(self, active_model: &mut A);
}

impl<A, F> ActiveModelUpdater<A> for F
    where A: ActiveModelTrait,
          F: FnOnce(&mut A), {
    fn update_active(self, active_model: &mut A) { (self)(active_model) }
}

pub trait UpdateActiveModel<Updater>
    where Updater: ActiveModelUpdater<Self>,
          Self: Sized + ActiveModelTrait, {
    fn update(&mut self, updater: Updater) { updater.update_active(self); }

    fn chain_update(mut self, updater: Updater) -> Self {
        updater.update_active(&mut self);
        self
    }
}

impl<T, A> UpdateActiveModel<A> for T
    where T: ActiveModelTrait + Sized,
          A: ActiveModelUpdater<T>, {}


pub struct UpdateWith<F: Field, A: Has<F>, T>(pub A::Ty, pub T);

impl<F: Field, A: Has<F> + ActiveModelTrait, T: IntoActiveModel<A>> IntoActiveModel<A> for UpdateWith<F, A, T> {
    fn into_active_model(self) -> A {
        let mut this = self.1.into_active_model();
        Has::set(&mut this, self.0);
        this
    }
}

impl<F, A, T> ActiveModelUpdater<A> for UpdateWith<F, A, T>
    where A: ActiveModelTrait + Has<F>,
          T: ActiveModelUpdater<A>,
          F: Field {
    fn update_active(self, active_model: &mut A) {
        Has::<F>::set(active_model, self.0);
        self.1.update_active(active_model);
    }
}


pub trait With<A>: Sized {
    fn with<F>(self,_:F, field: A::Ty) -> UpdateWith<F, A, Self>
        where F: Field,
        A:Has<F>
    {
        UpdateWith(field, self)
    }
}

impl<T: Sized,A> With<A> for T {}


has_field!(FieldOrder:order);
pub type UpdateWithOrder<A, T> = UpdateWith<FieldOrder, A, T>;