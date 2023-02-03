use crate::{range_limit::SizeStatus, RangeBound};

#[derive(Default)]
pub struct FixedSize<const S: usize>;

impl<const S: usize> std::fmt::Debug for FixedSize<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FixedSize")
            .field("require size", &S)
            .finish()
    }
}

impl<const S: usize> RangeBound for FixedSize<S> {
    fn match_range(input: usize) -> SizeStatus {
        match (input, S) {
            (i, s) if i == s => SizeStatus::Ok,
            (i, _) => SizeStatus::FIxSize(i),
        }
    }
}
