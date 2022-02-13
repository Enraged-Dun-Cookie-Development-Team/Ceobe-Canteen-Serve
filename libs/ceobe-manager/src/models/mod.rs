mod share_str;

use self::share_str::AShareString;

mod ws_msg;
pub type CachedId = AShareString;
pub type DataSource = AShareString;

pub use ws_msg::DataItem;
