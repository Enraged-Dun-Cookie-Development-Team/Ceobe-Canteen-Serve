pub mod serde;
pub mod limits;
pub mod range_limit;

pub trait RangeBound: Default {
    fn match_range(input: usize) -> SizeStatus;
}

pub enum SizeStatus {
    Ok,
    TooLarge(usize),
    TooSmall(usize),
    FIxSize(usize),
    Costom(Box<dyn std::error::Error>),
}

impl SizeStatus {
    pub fn costom<E: std::error::Error + 'static>(err: E) -> Self {
        let b = Box::new(err) as Box<dyn std::error::Error>;
        Self::Costom(b)
    }
}
