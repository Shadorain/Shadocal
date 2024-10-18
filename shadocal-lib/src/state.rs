use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Local, NaiveDate, NaiveTime};
use tokio::sync::RwLock;

use super::{
    db::{doc, CollectionT, Db},
    format::Format,
    Calendar, CalendarType, Event, InitToken, OToken, Profile,
};

pub struct State {
    pub calendars: RwLock<HashMap<String, Box<dyn Calendar>>>,
    pub db: Db,
}
impl State {
    pub async fn new(db: Db) -> Result<Self> {
        let mut calendars = HashMap::new();
        for profile in db.accounts().find(doc! {}).run()? {
            let Profile {
                email,
                refresh_token,
                ..
            } = profile?;
            println!("[INFO] Adding account: {email}");
            calendars.insert(
                email,
                CalendarType::Google
                    .init(Some(InitToken::Refresh(
                        refresh_token.expect("Token currently expected to never be None"),
                    )))
                    .await?,
            );
        }
        Ok(Self {
            calendars: calendars.into(),
            db,
        })
    }

    pub async fn new_calendar(&self, cal: CalendarType, token: Option<OToken>) -> Result<()> {
        let cal = cal.init(token.clone().map(InitToken::Access)).await?;
        let profile = cal.get_profile().await?.refresh_token(token);
        println!("Adding calendar: {}", profile.email);

        self.db.add_account(&profile)?;

        self.calendars.write().await.insert(profile.email, cal);
        Ok(())
    }

    pub async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        self.calendars
            .read()
            .await
            .get(&cal_id)
            .ok_or_else(|| anyhow!("No such calendar for id: {}", cal_id))?
            .get_event(cal_id, event_id)
            .await
    }
    pub async fn get_eventf<F: Format>(&self, cal_id: String, event_id: String) -> Result<String> {
        F::format(self.get_event(cal_id, event_id).await?).context("Failed to format event")
    }

    pub async fn list_events(&self, start: NaiveDate, end: NaiveDate) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for (_, cal) in self.calendars.read().await.iter() {
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

    pub async fn delete(&self, id: Option<String>) -> Result<()> {
        let mut cals = self.calendars.write().await;
        let query = if let Some(id) = id {
            cals.remove(&id).context("Could not find account")?;
            doc! {
                "id": { "$eq": id },
            }
        } else {
            cals.clear();
            doc! {}
        };
        self.db.accounts().delete_many(query)?;
        Ok(())
    }
}

fn to_datetime(date: NaiveDate) -> DateTime<Local> {
    date.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap())
        .and_local_timezone(Local)
        .earliest()
        .expect("Cant convert to local datetime")
}
