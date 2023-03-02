use bool_or::TrueOrError;
use ceobe_user::ToCeobeUser;
use futures::future::BoxFuture;
use axum::{body::{Body, BoxBody}, response::{Response, IntoResponse}, extract::FromRequestParts};
use mongo_migration::{mongo_connection::MongoDatabaseOperate, mongo_models::ceobe::user::models::UserMobId};
use resp_result::RespResult;
use tap::Tap;
use tower_http::auth::AsyncAuthorizeRequest;
use http::Request;
use tracing::{info, Instrument};
use tracing_unwrap::OptionExt;

use crate::{utils::mob_verify::get_mob_information, middleware::mob::MobIdInfo};

use super::error::MobVerifyError;

#[derive(Default, Debug, Clone, Copy)]
pub struct MobVerify;

impl AsyncAuthorizeRequest<Body> for MobVerify {
    type Future = BoxFuture<'static, Result<Request<Body>, Response>>;
    type RequestBody = Body;
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let result = 'auth: {
                let Some(mob_id) = get_mob_information(&request) else{
                    break 'auth Err(MobVerifyError::MobIdFieldNotFound)
                };

                let mob_id = mob_id.to_string();

                let (mut parts,body )= request.into_parts();
                let mongo = MongoDatabaseOperate::from_request_parts(&mut parts,&()).await.unwrap();
                let req = Request::from_parts(parts,body);

                let true = mongo.ceobe_user().user().is_exist_user(&mob_id).await.unwrap() else {
                    break 'auth Err(MobVerifyError::MobIdNotExist(mob_id))
                };

                info!(
                    user.mob_id = mob_id,
                );

                Ok(req.tap_mut(|req| {
                    req.extensions_mut()
                        .insert(MobIdInfo(UserMobId {mob_id}))
                        .expect_none_or_log("Mob Layer Exist")
                }))
            }.map_err(|err|RespResult::<(), _>::Err(err).into_response());
            
            result
        }.instrument(tracing::info_span!("mob_verify")))
    }
}