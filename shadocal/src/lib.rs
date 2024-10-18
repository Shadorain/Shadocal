mod ui;
pub use ui::App;
mod state;
pub use state::*;

pub const SHADOCAL_TITLE: &str = "Shadocal";
pub const SHADOCAL_TITLE_DESC: &str = "A blazingly fast, calendar event formatter webserver tool.";
pub const SHADOCAL_URL: &str = "https://github.com/Shadorain/Shadocal";
pub const SHADOCAL_VERSION: &str = env!("CARGO_PKG_VERSION");
