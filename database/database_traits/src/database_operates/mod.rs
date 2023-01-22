

mod db_op_impls;
pub mod sub_operate;

pub struct DatabaseOperate<C> {
    connect: C,
}

