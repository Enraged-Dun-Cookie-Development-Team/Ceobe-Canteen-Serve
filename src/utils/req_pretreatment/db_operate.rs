use std::marker::PhantomData;

use actix_web::web::Data;
use futures::Future;

use super::Pretreatment;
use crate::database::{traits::select::LoadFromDb, ServeDatabase};

pub struct DbOp<Select, P>(P, PhantomData<Select>);

impl<Select, P> Pretreatment for DbOp<Select, P>
where
    P: Pretreatment,
    Select: LoadFromDb,
    Select::Args: From<P::Resp>,
    P::Err: Into<Select::Err>,
{
    type Err = Select::Err;
    type Resp = Select::Target;

    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    fn call<'r>(
        req: &'r actix_web::HttpRequest, payload: &'r mut actix_http::Payload,
    ) -> Self::Fut {
        let db = req
            .app_data::<Data<ServeDatabase<sea_orm::DatabaseConnection>>>()
            .expect("Database Connect Not Found In AppData")
            .clone();
        let p_task = P::call(req, payload);
        async move {
            let args = p_task.await.map_err(Into::into)?;
            let res = Select::load(args.into(), &db).await?;
            Ok(res)
        }
    }
}
