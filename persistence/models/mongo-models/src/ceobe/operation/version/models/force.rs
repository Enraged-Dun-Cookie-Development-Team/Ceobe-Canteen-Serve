use semver::Version;
use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

#[derive(Debug, Serialize, Deserialize, Clone, TypedBuilder, PartialEq)]
#[builder(mutators(
    pub fn force_update(&mut self){
        self.force_update = true
    }
    pub fn set_force_update(&mut self,force:bool){
        self.force_update = force
    }
))]
pub struct ForceCtrl {
    #[builder(via_mutators)]
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
