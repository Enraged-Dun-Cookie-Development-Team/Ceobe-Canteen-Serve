use self::range_limit::RangeLimit;


pub mod header_info;
pub mod measureable;
pub mod range_limit;

pub type RangeLimitString<const L: usize, const H: usize> = RangeLimit<String, L, H>;
pub type MaxLimitString<const H: usize> = RangeLimitString<0, H>;

pub type RangeLimitVec<T, const L: usize, const H: usize> = RangeLimit<Vec<T>, L, H>;