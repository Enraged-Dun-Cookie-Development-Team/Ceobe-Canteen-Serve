use sea_orm::ActiveModelTrait;

pub trait ActiveModelUpdater {
    type ActiveModel: ActiveModelTrait;

    fn update_active(self, active_model: &mut Self::ActiveModel);
}

impl<A, F> ActiveModelUpdater for F 
    where A: ActiveModelTrait, 
          F: FnOnce(&mut A) 
{
    type ActiveModel = A;

    fn update_active(self, active_model: &mut Self::ActiveModel) {
        (self)(active_model)
    }
}


pub trait UpdateActiveModel<Updater>: Sized where Updater: ActiveModelUpdater {
    fn update(&mut self, updater: Updater) {
        updater.update_active(self);
    }

    fn chain_update(mut self, updater: Updater) -> Self {
        updater.update_active(&mut self);
        self
    }
}