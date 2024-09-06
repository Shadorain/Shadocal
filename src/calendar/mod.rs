use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use google_calendar::{
    calendar_list::CalendarList,
    events::Events,
    types::{MinAccessRole, OrderBy},
};

mod oauth;
use oauth::get_client;
mod event;
pub use event::*;

pub struct Calendar {
    events: Events,
    cal_list: CalendarList,
}
impl Calendar {
    pub async fn new() -> Result<Self> {
        let client = get_client().await?;
        Ok(Self {
            events: client.events(),
            cal_list: client.calendar_list(),
        })
    }

    pub async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        let cal = self
            .cal_list
            .list(0, MinAccessRole::Reader, "", false, true)
            .await?
            .body;
        let cal = cal
            .iter()
            .find(|cal| cal.id == cal_id)
            .context("Could not find specified calendar")?;
        Ok(Event::convert(
            self.events.get(&cal.id, &event_id, 0, "").await?.body,
            cal.id.clone(),
        ))
    }

    pub async fn list_events(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
        hidden: bool,
    ) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for cal in self
            .cal_list
            .list(0, MinAccessRole::Reader, "", false, true)
            .await?
            .body
        {
            events.extend(
                // Documentation: [API Reference](https://developers.google.com/calendar/api/v3/reference/events/list)
                self.events
                    .list(
                        &cal.id,
                        "",
                        0,
                        0,
                        OrderBy::StartTime,
                        "",
                        &[],
                        "",
                        &[],
                        false,
                        hidden,
                        true,
                        &end.to_rfc3339(),
                        &start.to_rfc3339(),
                        "",
                        "",
                    )
                    .await?
                    .body
                    .into_iter()
                    .map(|e| Event::convert(e, cal.id.clone())),
            );
        }
        Ok(events)
    }
}
