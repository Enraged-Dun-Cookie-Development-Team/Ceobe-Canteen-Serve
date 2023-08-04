use sea_orm::ActiveModelTrait;

pub trait ActiveModelUpdater<A: ActiveModelTrait> {
    fn update_active(self, active_model: &mut A);
}

impl<A, F> ActiveModelUpdater<A> for F
where
    A: ActiveModelTrait,
    F: FnOnce(&mut A),
{
    fn update_active(self, active_model: &mut A) { (self)(active_model) }
}

pub trait UpdateActiveModel<Updater>
where
    Updater: ActiveModelUpdater<Self>,
    Self: Sized + ActiveModelTrait,
{
    fn update(&mut self, updater: Updater) { updater.update_active(self); }

    fn chain_update(mut self, updater: Updater) -> Self {
        updater.update_active(&mut self);
        self
    }
}

impl<T, A> UpdateActiveModel<A> for T
where
    T: ActiveModelTrait + Sized,
    A: ActiveModelUpdater<T>,
{
}
