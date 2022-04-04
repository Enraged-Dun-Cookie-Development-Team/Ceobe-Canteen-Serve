use crate::database::model_register::SqlModelRegister;

use super::ModelRegister;

pub struct SqlRegister<F>(F)
where
    F: FnOnce(SqlModelRegister) -> SqlModelRegister;

impl<F> ModelRegister for SqlRegister<F>
where
    F: FnOnce(SqlModelRegister) -> SqlModelRegister,
{
    fn register_sql(self, db: SqlModelRegister) -> SqlModelRegister {
        self.0(db)
    }
}

pub fn as_sql_register<F: FnOnce(SqlModelRegister) -> SqlModelRegister>(
    fun: F,
) -> SqlRegister<F> {
    SqlRegister(fun)
}