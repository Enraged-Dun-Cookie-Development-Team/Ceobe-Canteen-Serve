mod model;
mod controller;


pub use controller::Controller;
pub use model::ModelRegister;
pub use model::as_mongo_register;
pub use model::as_sql_register;
pub use model::MongoRegister;
pub use model::SqlRegister;