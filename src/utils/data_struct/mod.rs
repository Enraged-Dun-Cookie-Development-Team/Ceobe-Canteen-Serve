use range_limit::{
    limits::{double_end_limit::DoubleEndLimit, max_limit::MaxLimit},
    RangeBoundLimit,
};

pub mod header_info;

pub type RangeLimitString<const L: usize, const H: usize> =
    RangeBoundLimit<String, DoubleEndLimit<L, H>>;
pub type MaxLimitString<const H: usize> = RangeBoundLimit<String, MaxLimit<H>>;
