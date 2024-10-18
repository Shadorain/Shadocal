use std::sync::{Arc, LazyLock};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use gcal_rs::{
    types::{
        EventCalendarDate, EventConferenceData, EventStatus as GEventStatus,
        EventType as GEventType,
    },
    CalendarListClient, EventClient, GCalClient, OAuth,
};
pub use gcal_rs::{OAuthRequest, OToken};

use super::{calendar_trait, Calendar, Event, EventStatus, EventType, InitToken, Profile};

pub static OAUTH: LazyLock<Arc<OAuth>> = LazyLock::new(|| {
    let client_id = std::env::var("GOOGLE_CLIENT_ID")
        .expect("[ERR] Missing the GOOGLE_CLIENT_ID environment variable.");
    let client_secret = std::env::var("GOOGLE_CLIENT_SECRET")
        .expect("[ERR] Missing the GOOGLE_CLIENT_SECRET environment variable.");
    let (ip, port) = crate::ip_port();
    OAuth::new(
        client_id,
        client_secret,
        format!("http://{ip}:{port}/account/auth/authenticate"),
    )
    .into()
});

pub struct GoogleCalendar {
    client: Arc<GCalClient>,
    calendars: CalendarListClient,
    events: EventClient,
}

impl GoogleCalendar {
    pub async fn new(token: Option<InitToken>) -> Result<Self> {
        let token = match token.context("Google calendar token is required")? {
            InitToken::Access(tok) => tok,
            InitToken::Refresh(refresh) => OAUTH.exhange_refresh(refresh).await?,
        };

        println!("Token: {token:?}");
        let client = GCalClient::new(token, Some(OAUTH.clone()))?;
        let (calendars, events) = client.clone().clients();
        Ok(Self {
            client,
            calendars,
            events,
        })
    }
}

#[calendar_trait]
impl Calendar for GoogleCalendar {
    async fn get_event(&self, cal_id: String, event_id: String) -> Result<Event> {
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

    async fn get_profile(&self) -> Result<Profile> {
        Ok(self
            .client
            .get(None, gcal_rs::UserInfo::default())
            .await?
            .json::<gcal_rs::UserInfo>()
            .await?
            .into())
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
impl From<gcal_rs::UserInfo> for Profile {
    fn from(value: gcal_rs::UserInfo) -> Self {
        Self {
            id: value.id,
            email: value.email,
            name: value.name,
            picture_link: value.picture,
            refresh_token: None,
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
