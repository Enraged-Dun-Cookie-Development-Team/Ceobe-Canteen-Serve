use crate::{
    RangeBoundLimit,
    range_limit::{RangeBound, SizeStatus},
};

#[derive(Debug, Default)]
pub struct NoLimit;

impl RangeBound for NoLimit {
    fn match_range(_: usize) -> SizeStatus { SizeStatus::Ok }
}

pub type NoRangeLimit<T> = RangeBoundLimit<T, NoLimit>;
