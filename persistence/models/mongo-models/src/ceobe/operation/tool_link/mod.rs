pub use check::tool_link_checker::ToolLinkChecker;

mod check;
pub mod models;

pub type Uncheck = checker::Uncheck<ToolLinkChecker>;
pub type Checked = checker::Checked<ToolLinkChecker>;

pub use check::CheckError;
