use crate::utils::mvc_utils::as_mongo_register;

pub mod check;
pub mod mongo_db;
mod register;

pub mod preludes {
    pub use super::{check::*, mongo_db::*};
}

crate::generate_model_register!(
    MansionModel,
    as_mongo_register(register::loading_model)
);
