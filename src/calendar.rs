use std::path::PathBuf;

use anyhow::Result;
use chrono::{DateTime, Local};
use directories::ProjectDirs;
use gcal::*;
use http_client::h1::H1Client;

mod oauth;

pub struct Calendar {
    cal_client: CalendarListClient<H1Client>,
    event_client: EventClient<H1Client>,
}
impl Calendar {
    pub async fn new() -> Result<Self> {
        let http_client = H1Client::new();
        let access_key = oauth::access_token(&http_client, data_directory()?).await;
        let client = GCalClient::new(http_client, access_key)?;
        Ok(Self {
            cal_client: CalendarListClient::new(client.clone()),
            event_client: EventClient::new(client),
        })
    }

    pub async fn get_events(
        &self,
        start: DateTime<Local>,
        end: DateTime<Local>,
        hidden: bool,
    ) -> Result<Vec<Event>> {
        let mut events = Vec::new();
        for cal in self
            .cal_client
            .list(hidden, CalendarAccessRole::Reader)
            .await?
        {
            events.extend(self.event_client.list(cal.id, start, end).await?)
        }
        Ok(events)
    }
}

pub fn data_directory() -> Result<PathBuf> {
    Ok(
        if let Some(proj_dirs) = ProjectDirs::from("com", "shadorain", "gcal") {
            proj_dirs.data_local_dir().to_path_buf()
        } else {
            return Err(anyhow::anyhow!(
                "Unable to find data directory for ShadoGCal"
            ));
        },
    )
}
