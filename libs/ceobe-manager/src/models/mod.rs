mod share_str;
use std::sync::Arc;
use self::share_str::AShareString;

mod ws_msg;
pub type CachedId = AShareString;
pub type DataSource = Arc<String>;

pub use ws_msg::DataItem;

