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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
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
