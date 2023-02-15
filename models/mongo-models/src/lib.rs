pub mod bakery;
pub mod ceobe;

use chrono::Local;
pub use mongo_connection;
pub use mongodb;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordUnit {
    pub create_at: DateTime,
    pub modify_at: DateTime,
    pub delete_at: Option<DateTime>,
}

impl Default for RecordUnit {
    fn default() -> Self { Self::new() }
}

impl RecordUnit {
    pub fn new() -> Self {
        let now = now();
        Self {
            create_at: now,
            modify_at: now,
            delete_at: None,
        }
    }

    pub fn modify(&mut self) { self.modify_at = now() }

    pub fn delete(&mut self) { self.delete_at.replace(now()); }
}

fn now() -> DateTime {
    let now = Local::now();
    DateTime::from_chrono(now)
}
