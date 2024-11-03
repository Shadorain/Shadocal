use dioxus::prelude::*;
use shadocal_lib::OToken;

#[cfg(feature = "server")]
use super::AppState;

#[server(NewCalendar)]
pub async fn new_calendar(token: OToken) -> Result<(), ServerFnError> {
    println!("New calendar {token:?}");
    let FromContext(state): FromContext<AppState> = extract().await?;
    state
        .new_calendar(shadocal_lib::CalendarType::Google, Some(token))
        .await
        .map_err(ServerFnError::new)
}
