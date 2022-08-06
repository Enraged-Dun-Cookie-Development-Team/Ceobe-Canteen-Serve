use std::{borrow::Cow, collections::HashSet};

use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{encode::encode, error::VerifyResult};

pub trait ModifyState: Sized {
    fn get_last_modify_time(&self) -> Cow<'_, NaiveDateTime>;

    type Identify: Serialize + Clone;

    fn get_identify(&self) -> Cow<'_, Self::Identify>;

    fn verify_modify(self, modify_since: &NaiveDateTime) -> CacheState<Self> {
        let modify_time = self.get_last_modify_time();
        if modify_time.as_ref() <= modify_since {
            CacheState::NotModify
        }
        else {
            CacheState::Update(self)
        }
    }

    fn verify_entity_tag(
        self, entity_tag: &HashSet<String>,
    ) -> VerifyResult<CacheState<Self>> {
        let self_identify = self.get_identify();
        let hashed = encode(&self_identify)?;
        if entity_tag.contains(&hashed) {
            Ok(CacheState::NotModify)
        }
        else {
            Ok(CacheState::Update(self))
        }
    }

    fn get_entity_tag(&self) -> VerifyResult<String> {
        let id = self.get_identify();
        encode(&id)
    }
}

pub enum CacheState<T> {
    NotModify,
    Update(T),
}
