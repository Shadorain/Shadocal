use std::sync::LazyLock;

mod ui;
use tokio::sync::RwLock;
pub use ui::App;
mod server;
pub use server::*;
mod state;
pub use state::*;

mod config;
pub use config::Config;

pub const SHADOCAL_TITLE: &str = "Shadocal";
pub const SHADOCAL_TITLE_DESC: &str = "A blazingly fast, calendar event formatter webserver tool.";
pub const SHADOCAL_URL: &str = "https://github.com/Shadorain/Shadocal";
pub const SHADOCAL_VERSION: &str = env!("CARGO_PKG_VERSION");

pub static SHADOCAL: LazyLock<RwLock<Shadocal>> = LazyLock::new(|| Shadocal::default().into());
