use std::fmt::Display;

pub mod cookie_list;
pub mod fetcher;

pub fn concat_key(key: impl Display, concat_str: impl Display) -> String {
    format!("{key}{concat_str}")
}
