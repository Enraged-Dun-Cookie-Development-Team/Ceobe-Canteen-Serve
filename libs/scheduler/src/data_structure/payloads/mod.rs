mod data_source;
mod data_source_args;
mod groups;
mod platform;
use serde::Serialize;
use smallstr::SmallString;

pub type GroupName = SmallString<[u8; 32]>;
pub type SourceType = SmallString<[u8; 32]>;
pub type PlatformName = SmallString<[u8; 16]>;

pub type ParamName = SmallString<[u8; 16]>;
pub type StrArgument = SmallString<[u8; 16]>;

use std::time::Duration;

pub use data_source::{DataSource, DataSourceList};
pub use data_source_args::{DataSourceArg, DataSourceArgs};
pub use groups::{Group, Groups};
pub use platform::{PlatformConfigure, Platforms};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, TypedBuilder)]
pub struct CeobeDunConfigure {
    #[builder(setter(transform = |interval: Duration| interval.as_millis()))]
    default_interval_milliseconds: u128,
    #[builder(default, setter(
        transform =|groups: impl IntoIterator<Item = Group>|
            groups
            .into_iter()
            .collect()
        )
    )]
    groups: Groups,
    #[builder(default, setter(
        transform = |value: impl IntoIterator<Item = (impl Into<PlatformName>, PlatformConfigure)>|
            Some(
                value
                .into_iter()
                .map(|(k,v)|(k.into(), v))
                .collect()
            )
        )
    )]
    platform: Option<Platforms>,
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use super::{CeobeDunConfigure, DataSource, Group, PlatformConfigure};
    #[test]
    fn test_build() {
        let config = CeobeDunConfigure::builder()
            .default_interval_milliseconds(Duration::from_millis(256))
            .groups([Group::builder()
                .name("121212")
                .data_source([
                    DataSource::builder().ty("bili").build(),
                    DataSource::builder()
                        .ty("weibo")
                        .arg([("aa", false.into()), ("cc", "str".into())])
                        .build(),
                ])
                .build()])
            .platform([(
                "bilibili",
                PlatformConfigure::builder()
                    .min_request_interval_microsecond(Duration::from_millis(
                        256,
                    ))
                    .build(),
            )])
            .build();

        let serde_str =  serde_json::to_string_pretty(&config).unwrap();
        println!("{serde_str}");
    }
}
