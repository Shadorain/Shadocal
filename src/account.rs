use anyhow::Result;

use crate::{Calendar, GoogleCalendar};

pub struct Account<T: Calendar = GoogleCalendar> {
    calendar: T,
    token: String,
}

impl<T: Calendar> Account<T> {
    pub async fn new() -> Result<Self> {
        let calendar = T::new().await?;
        Ok(Self {
            calendar,
            token: "".to_string(),
        })
    }
}
