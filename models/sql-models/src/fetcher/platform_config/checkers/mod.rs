pub mod platform_config_data;

use thiserror::Error;

use status_err::{ErrPrefix, StatusErr, HttpCode};
pub use CheckError::*;

#[derive(Debug, Error)]
pub enum CheckError {
    
}

impl StatusErr for CheckError {
    fn prefix(&self) -> ErrPrefix {
        match self {
            
        }
    }

    fn code(&self) -> u16 {
        match self {
            
        }
    }

    fn http_code(&self) -> HttpCode {
        match self {
            
        }
    }
}
