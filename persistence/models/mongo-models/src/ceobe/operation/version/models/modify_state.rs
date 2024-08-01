use std::borrow::Cow;

use modify_cache::ModifyState;
use semver::Version;

use crate::ceobe::operation::version::models::{
    ReleasePlatform, ReleaseVersion,
};

impl ModifyState for ReleaseVersion {
    type Identify = (Version, ReleasePlatform);

    fn get_identify(&self) -> Cow<'_, Self::Identify> {
        Cow::Owned((self.version.clone(), self.platform))
    }
}
