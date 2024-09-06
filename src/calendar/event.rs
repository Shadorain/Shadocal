#![allow(dead_code)]
use google_calendar::types::{self, ConferenceData};

#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub cal_id: String,
    pub id: String,

    pub status: String,
    pub title: String,
    pub description: Option<String>,

    pub start: String,
    pub end: Option<String>,

    pub attendees: Option<Vec<String>>,

    pub location: Option<String>,
    pub link: Option<(&'static str, String)>,
    pub cal_link: Option<String>,
}

impl Event {
    pub fn convert(value: types::Event, cal_id: String) -> Self {
        let event_type = EventType::from(value.event_type.as_str());
        let start = get_date(value.start).expect("Failed to parse start date");
        let end = (!value.end_time_unspecified)
            .then(|| get_date(value.end))
            .flatten();
        let attendees = (!value.attendees.is_empty()).then_some(
            value
                .attendees
                .into_iter()
                .map(|a| {
                    if a.display_name.is_empty() {
                        a.email
                    } else {
                        a.display_name
                    }
                })
                .collect::<Vec<_>>(),
        );
        Self {
            event_type,
            cal_id,
            id: value.id,
            status: make_ascii_titlecase(value.status),
            title: value.summary,
            description: empty_or(value.description),
            start,
            end,
            attendees,
            location: empty_or(value.location),
            link: get_link(value.conference_data),
            cal_link: empty_or(value.html_link),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    Meeting,
    Birthday,
    FocusTime,
    OutOfOffice,
}
impl From<&str> for EventType {
    fn from(value: &str) -> Self {
        match value {
            "birthday" => Self::Birthday,
            "focusTime" => Self::FocusTime,
            "outOfOffice" => Self::OutOfOffice,
            _ => Self::Meeting,
        }
    }
}

fn get_date(date: Option<types::EventDateTime>) -> Option<String> {
    let date = date?;
    date.date_time
        .map(|d| d.with_timezone(&chrono::Local).to_rfc3339())
        .or(date.date.map(|d| d.to_string()))
}

fn empty_or(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn make_ascii_titlecase(mut s: String) -> String {
    // let mut s = s.to_owned();
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    s
}

fn get_link(conf_data: Option<ConferenceData>) -> Option<(&'static str, String)> {
    let e = conf_data?.entry_points.swap_remove(0);
    Some((
        match e.entry_point_type.as_str() {
            "more" => "by Tel Link",
            "phone" => "by Phone",
            _ => "Meeting",
        },
        e.uri,
    ))
}
