use std::{marker::PhantomData, sync::Arc};

use futures::Future;
use rresult::ErrorCode;

use crate::database::{traits::select::LoadFromDb, ServeDatabase};

use super::Pretreatment;

pub struct DbOp<Select, P>(P, PhantomData<Select>);

impl<Select, P> Pretreatment for DbOp<Select, P>
where
    P: Pretreatment,
    Select: LoadFromDb<Args = P::Resp>,
    P::Err: Into<Select::Err>,
{
    type Fut = impl Future<Output = Result<Self::Resp, Self::Err>>;

    type Resp = Select::Target;

    type Err = Select::Err;

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
            let args = p_task.await.map_err(Into::into)?;
            let res = Select::load(args, &db).await?;
            Ok(res)
        }
    }
}
