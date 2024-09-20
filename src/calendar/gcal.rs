use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use gcal_rs::{CalendarListClient, EventClient, GCalClient};

use super::{oauth::*, Calendar, Event};

pub struct GoogleCalendar {
    events: EventClient,
    calendars: CalendarListClient,
}
impl GoogleCalendar {
    pub async fn new(access_token: String) -> Result<Self> {
        let (events, calendars) = GCalClient::new(access_token).clients();
        Ok(Self { events, calendars })
    }
}

impl Calendar for GoogleCalendar {
    async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        let cal = self
            .calendars
            .list(0, MinAccessRole::Reader, "", false, true)
            .await?;
        let cal = cal
            .iter()
            .find(|cal| cal.id == cal_id)
            .context("Could not find specified calendar")?;
        Ok(self.events.get(&cal.id, &event_id, 0, "").await)
    }

    async fn list_events(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
        hidden: bool,
    ) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for cal in self
            .calendars
            .list(0, MinAccessRole::Reader, "", false, true)
            .await?
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
