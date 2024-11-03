mod app;
pub use app::App;

mod routes;
use routes::Route;

mod state;
use state::*;

use super::*;

pub type UIState = dioxus::signals::Signal<state::UIState>;
