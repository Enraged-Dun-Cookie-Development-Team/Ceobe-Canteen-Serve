use crate::utils::mvc_utils::as_mongo_register;

mod register;
pub mod check;
pub mod mongo_db;


pub mod preludes{
    pub use super::check::*;
    pub use super::mongo_db::*;
}


crate::generate_model_register!(
    MansionModel,
    as_mongo_register(register::loading_model)
);

