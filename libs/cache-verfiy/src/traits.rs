use std::borrow::Cow;

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{encode::encode, error::VerifyResult};

pub trait ModifyState: Sized {
    fn get_last_modify_time(&self) -> &NaiveDateTime;

    type Identify: Serialize + Clone + PartialEq + Eq;

    fn get_identify(&self) -> Cow<'_, Self::Identify>;

    fn verify_modify(
        self, modify_since: &NaiveDateTime,
    ) -> CacheState<Self, NaiveDateTime> {
        let modify_time = self.get_last_modify_time();
        if modify_time <= modify_since {
            CacheState::NotModify
        }
        else {
            let time = modify_time.clone();
            CacheState::Update(self, time)
        }
    }

    fn verify_entity_tag(
        self, entity_tag: &str,
    ) -> VerifyResult<CacheState<Self, String>> {
        let self_identify = self.get_identify();
        let hashed = encode(&self_identify)?;
        if hashed == entity_tag {
            Ok(CacheState::NotModify)
        }
        else {
            Ok(CacheState::Update(self, hashed))
        }
    }
}

pub enum CacheState<T, F> {
    NotModify,
    Update(T, F),
}
