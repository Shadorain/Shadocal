use std::sync::LazyLock;

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use gcal_rs::{
    types::{
        EventCalendarDate, EventConferenceData, EventStatus as GEventStatus,
        EventType as GEventType,
    },
    CalendarListClient, EventClient, GCalClient, OAuth, OToken,
};
use tokio::sync::RwLock;

use super::{calendar_trait, Calendar, Event, EventStatus, EventType};

static OAUTH: LazyLock<RwLock<OAuth>> = LazyLock::new(|| {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("[ERR] Missing the GOOGLE_CLIENT_ID environment variable.");
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .expect("[ERR] Missing the GOOGLE_CLIENT_SECRET environment variable.");
    let (ip, port) = crate::ip_port();
    OAuth::new(
        client_id,
        client_secret,
        format!("http://{ip}:{}/auth", port + 1),
    )
    .into()
});

pub struct GoogleCalendar {
    calendars: CalendarListClient,
    events: EventClient,
    token: RwLock<OToken>,
}

impl GoogleCalendar {
    pub async fn new(token: Option<String>) -> Result<Self> {
        let token = if let Some(token) = token {
            OAUTH.read().await.exhange_refresh(token).await
        } else {
            OAUTH.write().await.naive().await
        }?;

        let (calendars, events) = GCalClient::new(token.access.clone())?.clients();
        Ok(Self {
            calendars,
            events,
            token: token.into(),
        })
    }
}

#[calendar_trait]
impl Calendar for GoogleCalendar {
    async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
        OAUTH
            .read()
            .await
            .refresh(&mut *self.token.write().await)
            .await?;

        let cal = self
            .calendars
            .list(false, gcal_rs::CalendarAccessRole::Reader)
            .await?;
        let cal = cal
            .iter()
            .find(|cal| cal.id == cal_id)
            .context("Could not find specified calendar")?;
        Ok(self.events.get(cal.id.clone(), event_id).await?.into())
    }

    async fn list_events(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Result<Vec<Event>> {
        OAUTH
            .read()
            .await
            .refresh(&mut *self.token.write().await)
            .await?;

        let mut events = Vec::new();
        for cal in self
            .calendars
            .list(false, gcal_rs::CalendarAccessRole::Reader)
            .await?
        {
            events.extend(
                self.events
                    .list(cal.id, start, end)
                    .await?
                    .into_iter()
                    .map(|e| e.into()),
            );
        }
        Ok(events)
    }

    async fn token(&self) -> Option<String> {
        self.token.read().await.refresh.clone()
    }
}

impl From<gcal_rs::Event> for Event {
    fn from(value: gcal_rs::Event) -> Self {
        let event_type = EventType::from(value.event_type);
        let status = EventStatus::from(value.status);
        let start = date(value.start).expect("Failed to parse start date");
        let end = (!value.end_time_unspecified)
            .then(|| date(value.end))
            .flatten();
        let attendees = (!value.attendees.is_empty()).then_some(
            value
                .attendees
                .into_iter()
                .map(|a| {
                    if let Some(dn) = a.display_name {
                        dn
                    } else {
                        a.email
                    }
                })
                .collect::<Vec<_>>(),
        );
        Self {
            event_type,
            cal_id: value.calendar_id,
            id: value.id,
            status,
            title: value.summary,
            description: value.description,
            start,
            end,
            attendees,
            location: value.location,
            link: link(value.conference_data),
            cal_link: Some(value.html_link),
        }
    }
}

fn date(date: EventCalendarDate) -> Option<String> {
    date.date_time.or(date.date.map(|d| d.to_string()))
}
fn link(mut conf_data: EventConferenceData) -> Option<(&'static str, String)> {
    if !conf_data.entry_points.is_empty() {
        return Some(("Meeting", conf_data.entry_points.swap_remove(0).label?));
    }
    None
}

impl From<GEventStatus> for EventStatus {
    fn from(value: GEventStatus) -> Self {
        match value {
            GEventStatus::Confirmed => EventStatus::Confirmed,
            GEventStatus::Tentative => EventStatus::Tentative,
            GEventStatus::Cancelled => EventStatus::Cancelled,
        }
    }
}
impl From<GEventType> for EventType {
    fn from(value: GEventType) -> Self {
        match value {
            GEventType::Default => EventType::Meeting,
            GEventType::OutOfOffice => EventType::OutOfOffice,
            GEventType::FocusTime => EventType::FocusTime,
            GEventType::WorkingLocation => EventType::Meeting,
        }
    }
}
