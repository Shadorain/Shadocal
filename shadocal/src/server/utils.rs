use dioxus::prelude::*;

#[server(NewCalendar)]
pub async fn new_calendar(email: String) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::info!("New Calendar: {}", email);
    crate::SHADOCAL
        .write()
        .await
        .new_calendar(email, shadocal_lib::CalendarType::Google)
        .await
        .map_err(ServerFnError::new)
}
