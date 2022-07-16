use range_limit::{
    limits::max_limit::MaxLimit,
    RangeBoundLimit,
};

pub mod header_info;

pub type MaxLimitString<const H: usize> =
    RangeBoundLimit<String, MaxLimit<H>>;
