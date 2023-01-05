use crate::error;

pub mod limits;
pub mod range_limit_core;
pub mod serde;

pub trait RangeBound: Default {
    fn match_range(input: usize) -> SizeStatus;
}

pub enum SizeStatus {
    Ok,
    TooLarge(usize),
    TooSmall(usize),
    FIxSize(usize),
}

impl SizeStatus {
    pub fn to_result(self,size:usize) -> Result<(),error::Error> {
        match self {
            SizeStatus::Ok => Ok(()),
            SizeStatus::TooLarge(require) => {
                Err(error::Error::TooLarge { require, get: size })
            }
            SizeStatus::TooSmall(require) => {
                Err(error::Error::TooSmall { require, get: size })
            }
            SizeStatus::FIxSize(s) => {
                Err(error::Error::FixSize {
                    require: s,
                    get: size,
                })
            }
        }
    }
}
