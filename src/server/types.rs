use chrono::{Datelike, Duration, Month, Months, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Occurrence {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct List {
    pub occurence: Occurrence,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    pub start: String,
    #[serde(skip_deserializing)]
    pub end: Option<String>,
}
impl List {
    pub fn extract(&self) -> Option<(NaiveDate, NaiveDate)> {
        let year = self.year.unwrap_or(Utc::now().year());
        let start = match self.occurence {
            Occurrence::Daily => {
                NaiveDate::parse_from_str(&format!("{} {}", self.start, year), "%a, %b %-d %Y").ok()
            }
            Occurrence::Weekly => {
                NaiveDate::from_isoywd_opt(year, extract_number(&self.start)?, Weekday::Sun)
            }
            Occurrence::Monthly => NaiveDate::from_ymd_opt(
                year,
                self.start.parse::<Month>().ok()?.number_from_month(),
                1,
            ),
        }?;
        Some((
            start,
            match self.occurence {
                Occurrence::Daily => start + Duration::days(1),
                Occurrence::Weekly => start + Duration::weeks(1),
                Occurrence::Monthly => start + Months::new(1),
            },
        ))
    }
}
fn extract_number(input: &str) -> Option<u32> {
    for part in input.split_whitespace() {
        // Try to parse each part to a number
        if let Ok(number) = part.parse::<u32>() {
            return Some(number);
        }
    }
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Get {
    pub cal_id: String,
    pub event_id: String,
}
