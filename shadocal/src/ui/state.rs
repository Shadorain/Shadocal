use shadocal_lib::{Event, Profile};

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

#[derive(Debug)]
pub struct Account {
    profile: Profile,
    events: Vec<Event>,
}

#[derive(Debug)]
pub struct UIState {
    accounts: Vec<Account>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            accounts: Vec::new(),
        }
    }
}
