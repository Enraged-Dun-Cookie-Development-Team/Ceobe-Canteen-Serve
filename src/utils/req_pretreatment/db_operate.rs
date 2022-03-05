use std::{marker::PhantomData, sync::Arc};

use futures::Future;

use crate::database::{traits::select::LoadFromDb, ServeDatabase};

use super::Pretreatment;

// impl<Select> Pretreatment for Select
// where
//     Select: LoadFromDb<Args = ()>,
//     Select::Err: Into<actix_http::Error>,
// {
//
// }
pub enum DbOpErr<De, Pe> {
    Db(De),
    Pre(Pe),
}

impl<De, Pe> Into<actix_http::Error> for DbOpErr<De, Pe>
where
    De: Into<actix_http::Error>,
    Pe: Into<actix_http::Error>,
{
    fn into(self) -> actix_http::Error {
        match self {
            DbOpErr::Db(db) => db.into(),
            DbOpErr::Pre(pe) => pe.into(),
        }
    }
}

pub struct DbOp<Select, P>(P, PhantomData<Select>);

impl<Select, P> Pretreatment for DbOp<Select, P>
where
    P: Pretreatment,
    Select: LoadFromDb<Args = P::Resp>,
    Select::Err: Into<actix_http::Error>,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = Select::Target;

    type Err = DbOpErr<Select::Err, P::Err>;

    fn call<'r>(
        req: &'r actix_web::HttpRequest,
        payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let db = req
            .app_data::<Arc<ServeDatabase<sea_orm::DatabaseConnection>>>()
            .expect("Database Connect Not Found In AppData")
            .clone();
        let p_task = P::call(req, payload);
        async move {
            let args = p_task.await.map_err(DbOpErr::Pre)?;
            let res = Select::select_by(args, &db).await.map_err(DbOpErr::Db)?;
            Ok(res)
        }
    }
}
