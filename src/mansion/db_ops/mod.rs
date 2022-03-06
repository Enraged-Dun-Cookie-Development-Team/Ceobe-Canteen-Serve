//! 数据库操作部分代码位置
//! 

pub mod load_mansion;

pub(super) use db_entity::{
    each_mansion,
    mansion,
    inner_mansion,
    sea_orm_active_enums
};