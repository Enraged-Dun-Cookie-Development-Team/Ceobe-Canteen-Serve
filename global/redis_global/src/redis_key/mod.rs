pub mod cookie_list;
pub mod fetcher;

pub fn concat_key(key: &str, concat_str: &str) -> String {
    format!("{key}{concat_str}")
}
