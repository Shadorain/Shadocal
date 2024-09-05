use actix_web::{
    error::{self, PayloadError},
    get, web, Result,
};
use chrono::{Datelike, Duration, Month, Months, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};

use super::{format, State};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Occurrence {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct List {
    occurence: Occurrence,
    start: String,
    #[serde(skip_deserializing)]
    end: Option<String>,
}
impl List {
    fn extract(&self) -> anyhow::Result<(NaiveDate, NaiveDate)> {
        let now = Utc::now();
        let start = match self.occurence {
            Occurrence::Daily => NaiveDate::parse_from_str(
                &format!("{} {}", self.start, now.year()),
                "%a, %b %-d %Y",
            )?,
            Occurrence::Weekly => {
                NaiveDate::from_isoywd_opt(now.year(), extract_number(&self.start)?, Weekday::Sun)
                    .unwrap()
            }
            Occurrence::Monthly => NaiveDate::from_ymd_opt(
                now.year(),
                self.start.parse::<Month>()?.number_from_month(),
                1,
            )
            .unwrap(),
        };
        Ok((
            start,
            match self.occurence {
                Occurrence::Daily => start + Duration::days(1),
                Occurrence::Weekly => start + Duration::weeks(1),
                Occurrence::Monthly => start + Months::new(1),
            },
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Get {
    cal_id: String,
    event_id: String,
}

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/tana").service(get).service(list));
}

#[get("/get/{cal_id}/{event_id}")]
async fn get(_: web::Data<State>, get: web::Path<Get>) -> Result<String> {
    println!("{:?}", get);
    // data.get_events::<format::Tana>()
    //     .await
    //     .map_err(|err| error::ErrorFailedDependency(err.to_string()))
    todo!()
}

#[get("/list")]
async fn list(data: web::Data<State>, list: web::Json<List>) -> Result<String> {
    println!("{:?}", &list);
    let (start, end) = list.extract().or(Err(error::JsonPayloadError::Payload(
        PayloadError::EncodingCorrupted,
    )))?;
    data.get_events::<format::Tana>(start, end)
        .await
        .map_err(|err| error::ErrorFailedDependency(err.to_string()))
}

fn extract_number(input: &str) -> anyhow::Result<u32> {
    // Split the string on spaces and look for a numeric part
    for part in input.split_whitespace() {
        // Try to parse each part to a number
        if let Ok(number) = part.parse::<u32>() {
            return Ok(number);
        }
    }
    anyhow::bail!("Failed to extract number")
}
