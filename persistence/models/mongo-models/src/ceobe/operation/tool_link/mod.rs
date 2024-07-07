pub use check::tool_link_checker::ToolLinkChecker;

pub mod models;
mod check;

pub type Uncheck = checker::Uncheck<ToolLinkChecker>;
pub type Checked = checker::Checked<ToolLinkChecker>;

pub use check::CheckError;