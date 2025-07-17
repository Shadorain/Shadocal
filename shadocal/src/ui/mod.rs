mod app;
pub use app::App;

mod routes;
use routes::Route;

mod state;
use state::*;

mod icons;
use icons::*;

mod components;
use components::*;

use super::*;

pub type UIState = dioxus::signals::Signal<state::UIState>;
