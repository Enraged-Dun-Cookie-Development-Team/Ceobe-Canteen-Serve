pub mod limits;
pub mod range_limit;

pub trait RangeBound: Default {
    fn match_range(input: usize) -> SizeStatus;
}

pub enum SizeStatus {
    Ok,
    TooLarge(usize),
    TooSmall(usize),
}
