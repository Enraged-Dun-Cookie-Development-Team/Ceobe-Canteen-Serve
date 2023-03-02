use http::Request;
use axum::body::Body;
use std::borrow::Cow;

pub fn get_mob_information(req: &Request<Body>) -> Option<Cow<str>> {
    req.headers()
        .get("mob_id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| urlencoding::decode(s).ok())
}
