use std::ops::{Deref, DerefMut};

use shadocal_lib::State;

use super::Config;

#[derive(Default)]
pub struct Shadocal {
    state: State,
    config: Config,
}

impl Deref for Shadocal {
    type Target = State;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl DerefMut for Shadocal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl Shadocal {
    pub fn add_calendar(&mut self, id: String, token: String) {
        self.config
            .add_calendar(id, token)
            .expect("Failed to serialize calendar");
    }
}
