pub mod config_data;

use thiserror::Error;

use status_err::{ErrPrefix, StatusErr, HttpCode};
pub use CheckError::*;

#[derive(Debug, Error, StatusErr)]
pub enum CheckError {
    
}
