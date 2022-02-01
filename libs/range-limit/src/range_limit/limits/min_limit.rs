use crate::{
    range_limit::{RangeBound, SizeStatus},
    RangeBoundLimit,
};

#[derive(Default)]
pub struct MinLimit<const L: usize>;

impl<const L: usize> std::fmt::Debug for MinLimit<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MinLimit").field("min-bound", &L).finish()
    }
}

impl<const L: usize> RangeBound for MinLimit<L> {
    fn match_range(input: usize) -> SizeStatus {
        if input > L {
            SizeStatus::Ok
        } else {
            SizeStatus::TooSmall(L)
        }
    }
}

pub type MinRangeLimit<T, const MIN: usize> = RangeBoundLimit<T, MinLimit<MIN>>;
