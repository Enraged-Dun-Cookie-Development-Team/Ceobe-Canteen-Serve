use std::any::type_name;

use serde::Serialize;
use sha3::{Digest, Sha3_512};

use crate::error::VerifyResult;

pub fn encode<S: Serialize>(identify: &S) -> VerifyResult<String> {
    // serialize data
    let vec = bincode::serialize(identify)?;

    log::info!("提取 {} 特征中, len: {}", type_name::<S>(), vec.len());
    // sha3 hash
    let mut hash = Sha3_512::new();

    hash.update(vec);

    let out = hash.finalize();
    let encode = base64::encode(out);
    log::info!("提取 {} 特征完成, {}", type_name::<S>(), encode);

    Ok(encode)
}
