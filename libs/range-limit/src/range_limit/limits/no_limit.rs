use crate::{
    range_limit::{RangeBound, SizeStatus},
    RangeBoundLimit,
};

#[derive(Debug, Default)]
pub struct NoLimit;

impl RangeBound for NoLimit {
    fn match_range(_: usize) -> SizeStatus {
        SizeStatus::Ok
    }
}

pub type NoRangeLimit<T> = RangeBoundLimit<T, NoLimit>;
