use anyhow::Result;
use chrono::{DateTime, Local};

mod gcal;
mod types;

pub use async_trait::async_trait as calendar_trait;

pub use types::*;
// TODO: Find a solution for all calendar types...
pub use gcal::{OAuthRequest, OToken, OAUTH};

pub enum InitToken {
    Access(OToken),
    Refresh(String),
}

pub enum CalendarType {
    Google,
    Custom(Box<dyn Calendar>),
}
impl CalendarType {
    pub async fn init(self, token: Option<InitToken>) -> Result<Box<dyn Calendar>> {
        Ok(match self {
            Self::Google => Box::new(gcal::GoogleCalendar::new(token).await?),
            Self::Custom(c) => c,
        })
    }
}

#[calendar_trait]
pub trait Calendar: Send + Sync {
    async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event>;

    async fn list_events(&self, start: DateTime<Local>, end: DateTime<Local>)
        -> Result<Vec<Event>>;

    async fn get_profile(&self) -> Result<Profile>;
}
