use anyhow::Result;
use chrono::{DateTime, Local};

mod event;
mod gcal;
mod oauth;

pub use event::*;
pub use gcal::GoogleCalendar;

pub trait Calendar {
    async fn new(refresh_token: String) -> Result<Self>
    where
        Self: Sized;
    async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event>;

    async fn list_events(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
        hidden: bool,
    ) -> Result<Vec<Event>>;
}
