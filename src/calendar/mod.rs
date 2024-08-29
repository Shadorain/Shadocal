use anyhow::Result;
use chrono::{DateTime, Local};
use google_calendar::{
    calendar_list::CalendarList,
    events::Events,
    types::{Event, MinAccessRole, OrderBy},
};

mod oauth;
use oauth::get_client;

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

    pub async fn get_events(
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
                    .body,
            );
        }
        Ok(events)
    }
}
