#![allow(dead_code)]
use google_calendar::types;

pub struct Event {
    pub event_type: EventType,
    pub cal_id: Option<String>, // Required
    pub id: String,

    pub status: String,
    pub title: String,
    pub description: Option<String>,

    pub start: String,
    pub end: Option<String>, // Check endTimeUnspecified

    pub attendees: Option<Vec<String>>,

    pub location: Option<String>,
    pub link: Option<String>,
}

impl Event {
    pub fn set_calendar_id(&mut self, cal_id: String) {
        self.cal_id = Some(cal_id)
    }
}

impl From<types::Event> for Event {
    fn from(value: types::Event) -> Self {
        let event_type = EventType::from(value.event_type.as_str());
        make_ascii_titlecase(&value.status);
        let start = get_date(value.start).expect("Failed to parse start date");
        let end = (!value.end_time_unspecified)
            .then_some(get_date(value.end))
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
            cal_id: None,
            id: value.id,
            status: value.status,
            title: value.summary,
            description: empty_or(value.description),
            start,
            end,
            attendees,
            location: empty_or(value.location),
            link: empty_or(value.html_link),
        }
    }
}

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
        .map(|d| d.to_rfc3339())
        .or(date.date.map(|d| d.to_string()))
}

fn empty_or(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn make_ascii_titlecase(s: &str) -> String {
    let mut s = s.to_owned();
    if let Some(r) = s.get_mut(0..1) {
        r.make_ascii_uppercase();
    }
    s
}
