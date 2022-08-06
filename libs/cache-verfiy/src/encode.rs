use serde::Serialize;
use sha3::{Digest, Sha3_512};

use crate::error::VerifyResult;

pub fn encode<S: Serialize>(identify: &S) -> VerifyResult<String> {
    // serialize data
    let vec = bincode::serialize(identify)?;
    // sha3 hash
    let mut hash = Sha3_512::new();

    hash.update(vec);

    let out = hash.finalize();
    Ok(format!(
        "\"{}\"",
        String::from_utf8(out.to_ascii_lowercase())?
    ))
}
