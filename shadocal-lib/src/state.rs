use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveTime};

use super::{format::Format, Calendar, CalendarType, Event, InitToken, OToken};

#[derive(Default)]
pub struct State {
    pub calendars: HashMap<String, Box<dyn Calendar>>,
}
impl State {
    pub fn new() -> Self {
        Self {
            calendars: HashMap::new(),
        }
    }
    pub async fn configure(self, config: HashMap<String, String>) -> Result<Self> {
        let mut calendars = self.calendars;
        for (id, tok) in config.into_iter() {
            println!("[INFO] Adding calendar: {} with id: {}", &tok, &id);
            calendars.insert(
                id,
                CalendarType::Google
                    .init(Some(InitToken::Refresh(tok)))
                    .await?,
            );
        }
        Ok(Self { calendars })
    }
    pub async fn new_calendar(
        &mut self,
        cal: CalendarType,
        id: String,
        token: Option<OToken>,
    ) -> Result<()> {
        self.calendars
            .insert(id, cal.init(token.map(InitToken::Access)).await?);
        Ok(())
    }

    pub async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        self.get_cal(&cal_id)?.get_event(cal_id, event_id).await
    }
    pub async fn get_eventf<F: Format>(&self, cal_id: String, event_id: String) -> Result<String> {
        F::format(self.get_event(cal_id, event_id).await?).context("Failed to format event")
    }

    pub async fn list_events(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for (_, cal) in self.calendars.iter() {
            events.extend(
                cal.list_events(to_datetime(start), to_datetime(end))
                    .await?,
            );
        }
        Ok(events)
    }
    pub async fn list_eventsf<F: Format>(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<String> {
        F::format_list(self.list_events(start, end).await?).context("Failed to format event list")
    }

    fn get_cal(&self, cal_id: &str) -> Result<&dyn Calendar> {
        self.calendars
            .get(cal_id)
            .ok_or_else(|| anyhow!("No such calendar for id: {}", cal_id))
            .map(|x| &**x)
    }
}

fn to_datetime(date: NaiveDate) -> DateTime<Local> {
    date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_local_timezone(Local)
        .earliest()
        .expect("Cant convert to local datetime")
}
