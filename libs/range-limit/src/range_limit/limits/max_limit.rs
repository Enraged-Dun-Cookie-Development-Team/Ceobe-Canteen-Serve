use crate::{
    range_limit::{RangeBound, SizeStatus},
    RangeBoundLimit,
};

#[derive(Default)]
pub struct MaxLimit<const L: usize>;

impl<const L: usize> RangeBound for MaxLimit<L> {
    fn match_range(input: usize) -> crate::range_limit::SizeStatus {
        if input < L {
            SizeStatus::Ok
        } else {
            SizeStatus::TooLarge(L)
        }
    }
}

impl<const L: usize> std::fmt::Debug for MaxLimit<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MaxLimit").field("max-bound", &L).finish()
    }
}

pub type MaxRangeLimit<T, const MAX: usize> = RangeBoundLimit<T, MaxLimit<MAX>>;
