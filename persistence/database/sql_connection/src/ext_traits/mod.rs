pub mod active_or_set;
pub mod active_updater;
pub mod check_all_exist;
pub mod counter;
pub mod select_count;
pub mod soft_delete;

pub use active_updater::{ActiveModelUpdater, UpdateActiveModel};
pub use counter::{Count, CountNonZero, CountZero};
const COUNT_NAME: &str = "count";
