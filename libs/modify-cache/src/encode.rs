use std::any::type_name;

use serde::Serialize;
use sha3::{Digest, Sha3_512};
use tracing::info;

use crate::error::VerifyResult;

pub fn encode<S: Serialize>(identify: &S) -> VerifyResult<String> {
    // serialize data
    let vec = bincode::serialize(identify)?;

    // sha3 hash
    let mut hash = Sha3_512::new();

    hash.update(vec);

    let out = hash.finalize();
    let encode = base64::encode(out);
    info!(etag.ty = type_name::<S>(), etag = encode);

    Ok(encode)
}
