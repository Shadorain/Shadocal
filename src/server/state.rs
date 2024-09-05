use anyhow::Result;
use chrono::{DateTime, Local, NaiveDate, NaiveTime};

use super::{Calendar, Format};

pub struct State {
    cal: Calendar,
}
impl State {
    pub fn new(cal: Calendar) -> Self {
        Self { cal }
    }

    pub async fn get_events<T: Format>(&self, start: NaiveDate, end: NaiveDate) -> Result<String> {
        let events = self
            .cal
            .get_events(to_datetime(start), to_datetime(end), false)
            .await?;
        Ok(events
            .into_iter()
            .filter_map(|e| T::format(e))
            .collect::<Vec<_>>()
            .join(T::newline()))
    }
}

fn to_datetime(date: NaiveDate) -> DateTime<Local> {
    date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_local_timezone(Local)
        .earliest()
        .expect("Cant convert to local datetime")
}
