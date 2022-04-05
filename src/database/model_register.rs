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
