use crate::{
    range_limit::{RangeBound, SizeStatus},
    RangeBoundLimit,
};

#[derive(Default)]
pub struct DoubleEndLimit<const L: usize, const H: usize>;

impl<const L: usize, const H: usize> RangeBound for DoubleEndLimit<L, H> {
    fn match_range(input: usize) -> crate::range_limit::SizeStatus {
        if input > L && input < H {
            SizeStatus::Ok
        } else if input < L {
            SizeStatus::TooSmall(L)
        } else {
            SizeStatus::TooLarge(H)
        }
    }
}

impl<const L: usize, const H: usize> std::fmt::Debug for DoubleEndLimit<L, H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DoubleEndLimit")
            .field("max-bound", &H)
            .field("min-bound", &L)
            .finish()
    }
}

pub type RangeLimit<T, const MIN: usize, const MAX: usize> =
    RangeBoundLimit<T, DoubleEndLimit<MIN, MAX>>;
