mod stream_checker;
mod iter_checker;
mod range_checker;


pub mod iter_checkers{
    pub use super::iter_checker::{CheckedIter,IterChecker};
}