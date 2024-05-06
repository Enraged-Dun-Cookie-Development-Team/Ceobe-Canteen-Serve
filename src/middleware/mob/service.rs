use axum::{
    body::{Body, Body as BoxBody},
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use bool_or::TrueOrError;
use futures::future::BoxFuture;
use http::Request;
use persistence::{
    ceobe_cookie::ToCeobe,
    ceobe_user::{models::models::UserMobId, ToCeobeUser},
    mongodb::MongoDatabaseOperate,
};
use resp_result::RespResult;
use tap::Tap;
use tower_http::auth::AsyncAuthorizeRequest;
use tracing::{info, Instrument};
use tracing_unwrap::OptionExt;

use super::error::MobVerifyError;
use crate::{
    middleware::mob::MobIdInfo, utils::mob_verify::get_mob_information,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct MobVerify;

impl AsyncAuthorizeRequest<Body> for MobVerify {
    type Future = BoxFuture<'static, Result<Request<Body>, Response>>;
    type RequestBody = Body;
    type ResponseBody = BoxBody;

    fn authorize(&mut self, request: Request<Body>) -> Self::Future {
        Box::pin(
            async move {
                let result = 'auth: {
                    let Some(mob_id) = get_mob_information(&request)
                    else {
                        break 'auth Err(MobVerifyError::MobIdFieldNotFound);
                    };

                    let mob_id = mob_id.to_string();

                    let (mut parts, body) = request.into_parts();
                    let mongo = MongoDatabaseOperate::from_request_parts(
                        &mut parts,
                        &(),
                    )
                    .await
                    .unwrap();
                    let req = Request::from_parts(parts, body);

                    if let Err(err) = match mongo
                        .ceobe()
                        .user()
                        .property()
                        .is_exist_user(&mob_id)
                        .await
                        .map_err(|_| MobVerifyError::UserDatabaseOperateError)
                    {
                        Ok(exist) => exist,
                        Err(err) => break 'auth Err(err),
                    }
                    .true_or_with(|| {
                        MobVerifyError::MobIdNotExist(mob_id.clone())
                    }) {
                        break 'auth Err(err);
                    };

                    info!(user.mob_id = mob_id,);

                    Ok(req.tap_mut(|req| {
                        req.extensions_mut()
                            .insert(MobIdInfo(UserMobId { mob_id }))
                            .expect_none_or_log("Mob Layer Exist")
                    }))
                }
                .map_err(|err| RespResult::<(), _>::Err(err).into_response());

                result
            }
            .instrument(tracing::info_span!("mob_verify")),
        )
    }
}
