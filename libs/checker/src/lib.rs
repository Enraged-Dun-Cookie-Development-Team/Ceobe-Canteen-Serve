pub use check_fut::CheckFut;
pub use check_obj_macro::{check_obj, check_obj as check_gen};
pub use check_prehandle::{
    CheckExtract, FormCheckExtract, JsonCheckExtract, PathCheckExtract,
    QueryCheckExtract,
};
pub use checker_impls::CheckRefFut;
pub use lite_args::LiteArgs;
pub use require_check::{CheckRequire, ToCheckRequire};

pub use crate::checker::{Checker, LiteChecker, RefChecker};
pub use sync_check::SyncFuture;

mod check_fut;
mod check_prehandle;
mod checker;
mod checker_impls;
mod codegen;
mod lite_args;
pub mod prefabs;
mod require_check;

pub type Checked<C> = <C as Checker>::Checked;
pub type Uncheck<C> = <C as Checker>::Unchecked;
mod sync_check;