use sea_orm::{ActiveValue, Set};
use time_utils::chrono::NaiveDateTime;
use time_utils::{get_now_naive_date_time, get_zero_data_time};

pub trait SoftDelete {
    fn get_mut(&mut self) -> &mut ActiveValue<NaiveDateTime>;

    fn mut_by(&mut self, f: impl FnOnce(&mut ActiveValue<NaiveDateTime>)) {
        f(self.get_mut())
    }

    fn soft_remove(&mut self) {
        self.mut_by(|delete| {
            *delete = Set(get_now_naive_date_time());
        })
    }

    fn soft_recover(&mut self) {
        self.mut_by(|delete| {
            *delete = Set(get_zero_data_time());
        })
    }
}