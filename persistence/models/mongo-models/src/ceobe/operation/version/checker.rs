use checker::prefabs::no_check::NoCheck;
use crate::ceobe::operation::version::models::ReleaseVersion;

pub type ReleaseVersionChecker = NoCheck<ReleaseVersion>;