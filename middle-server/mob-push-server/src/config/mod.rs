pub(crate) mod app_info;

// use once_cell::sync::OnceCell;

// pub use self::app_info::MobPushConfig;

// #[allow(dead_code)]
// static PUSHER_CONFIG: OnceCell<MobPushConfig> = OnceCell::new();

// #[cfg(test)]
// pub(crate) fn load_from_test() {
//     let app = load_cfg();
//     PUSHER_CONFIG.set(app).expect("Config set")
// }
// // fn load_cfg() -> MobPushConfig {
// //     use std::{fs, path::Path};
// //     let vec = fs::read_to_string(Path::new("./config.toml")).expect("Config info not exist");
// //     toml::from_str(&vec).expect("Parse to Toml Failure")
// // }

// pub fn set_config(cfg: MobPushConfig) {
//     PUSHER_CONFIG.set(cfg).expect("Config Set")
// }

// pub(crate) fn get_config() -> &'static MobPushConfig {
//     #[cfg(test)]
//     {
//         PUSHER_CONFIG.get_or_init(load_cfg)
//     }
//     #[cfg(not(test))]
//     {
//         PUSHER_CONFIG.get().expect("Config Not Set")
//     }
// }

// pub fn load_config_from_default() {
//     PUSHER_CONFIG.set(load_cfg()).ok();
// }
