use semver::Version;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Clone, TypedBuilder,PartialEq)]
pub struct ForceCtrl {
    #[builder(setter(strip_bool))]
    force_update: bool,
    previous_force_version: Version,
}

#[cfg(test)]
mod test {
    use semver::Version;
    #[test]
    fn test_version_serde() {
        let ver = Version::new(1, 1, 1);
        let s = serde_json::to_string(&ver).unwrap();
        assert_eq!(s, "\"1.1.1\"")
    }
}
