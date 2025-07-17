use dioxus::prelude::*;
use shadocal_lib::Profile;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UIAction {
    LoginOpen,
}
impl UIAction {
    pub fn run(self) {
        match self {
            Self::LoginOpen => open::that("http://localhost:7117/account/auth/login").unwrap(),
        }
    }
}

#[derive(Debug, Default)]
pub struct UIState {
    pub current_profile: Option<Signal<Profile>>,
}

impl UIState {
    pub fn set_profile(&mut self, profile: Signal<Profile>) {
        self.current_profile = Some(profile);
    }
}
