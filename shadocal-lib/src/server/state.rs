use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveTime};

use super::{Calendar, CalendarType, Config, Event};

#[derive(Default)]
pub struct State {
    calendars: HashMap<String, Box<dyn Calendar>>,

    config: Option<Config>,
}
impl State {
    pub async fn new(config: Option<Config>) -> Result<Self> {
        let mut calendars = HashMap::new();
        if let Some(config) = &config {
            for (id, tok) in config.calendars.iter() {
                println!("[INFO] Adding calendar: {} with id: {}", tok, id);
                calendars.insert(
                    id.clone(),
                    CalendarType::Google.init(Some(tok.clone())).await?,
                );
            }
        }
        Ok(Self { calendars, config })
    }

    pub fn add_calendar(&mut self, id: String, cal: Box<dyn Calendar>) {
        self.calendars.insert(id, cal);
    }

    pub async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        self.get_cal(&cal_id)?.get_event(cal_id, event_id).await
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
