use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;


pub type EntityRegisterFn = fn (SqlModelRegister) -> SqlModelRegister;

static STATIC_MODEL_LIST_LOCK: AtomicBool = AtomicBool::new(false);

lazy_static::lazy_static! {
    static ref STATIC_MODEL_LIST: Mutex<Vec<EntityRegisterFn>> = Mutex::new(Vec::new());
}

pub(crate) fn static_register_model(func: EntityRegisterFn) {
    if STATIC_MODEL_LIST_LOCK.load(Ordering::Acquire) {
        panic!("should not call static_register_model after startup!");
    }
    STATIC_MODEL_LIST.try_lock().unwrap().push(func);
}

pub(super) fn get_model_list() -> Vec<EntityRegisterFn> {
    STATIC_MODEL_LIST_LOCK.store(true, Ordering::Release);
    let new_vec: Vec<EntityRegisterFn> = STATIC_MODEL_LIST.try_lock().unwrap().clone();
    new_vec
}

pub struct SqlModelRegister {
    db_backend: sea_orm::DatabaseBackend,
    schema: sea_orm::Schema,

    register_stmt: Vec<sea_orm::Statement>,
}

impl SqlModelRegister {
    pub(in crate::database) fn new<C: sea_orm::ConnectionTrait>(
        conn: &C,
    ) -> Self {
        Self {
            schema: sea_orm::Schema::new(conn.get_database_backend()),
            db_backend: conn.get_database_backend(),
            register_stmt: Vec::new(),
        }
    }

    pub fn register_model<M: sea_orm::EntityTrait>(
        mut self, model: M,
    ) -> Self {
        let mut stmt = self.schema.create_table_from_entity(model);
        stmt.if_not_exists();

        let stmt = self.db_backend.build(&stmt);

        self.register_stmt.push(stmt);
        self
    }

    pub(super) async fn register<C: sea_orm::ConnectionTrait>(
        self, db: &C,
    ) -> Result<(), sea_orm::DbErr> {
        for stmt in self.register_stmt {
            db.execute(stmt).await?;
        }
        Ok(())
    }
}
