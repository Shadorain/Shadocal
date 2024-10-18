use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub cal_id: String,
    pub id: String,

    pub status: EventStatus,
    pub title: String,
    pub description: Option<String>,

    pub start: String,
    pub end: Option<String>,

    pub attendees: Option<Vec<String>>,

    pub location: Option<String>,
    pub link: Option<(&'static str, String)>,
    pub cal_link: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    #[default]
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

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventStatus {
    #[default]
    Confirmed,
    Tentative,
    Cancelled,
}
impl From<&str> for EventStatus {
    fn from(value: &str) -> Self {
        match value {
            "tentative" => Self::Tentative,
            "cancelled" => Self::Cancelled,
            _ => Self::Confirmed,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq, Eq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub email: String,
    pub picture_link: String,

    pub refresh_token: Option<String>,
}
impl Profile {
    pub fn refresh_token(mut self, token: Option<super::OToken>) -> Profile {
        self.refresh_token = token.and_then(|t| t.refresh);
        self
    }
}
