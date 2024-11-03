use dioxus::prelude::*;
use server_fn::codec::Json;

use shadocal_lib::types::{Get, List};

#[cfg(feature = "server")]
use super::AppState;
#[cfg(feature = "server")]
use shadocal_lib::format;

#[server(
  name = TanaGet,
  prefix = "/tana",
  endpoint = "get",
  input = Json,
)]
pub async fn get(get: Get) -> Result<String, ServerFnError> {
    println!("Get: {:?}", get);
    let FromContext(state): FromContext<AppState> = extract().await?;
    state
        .get_eventf::<format::Tana>(get.cal_id, get.event_id)
        .await
        .map_err(ServerFnError::new)
}

#[server(
  name = TanaList,
  prefix = "/tana",
  endpoint = "list",
  input = Json,
)]
pub async fn list(list: List) -> Result<String, ServerFnError> {
    println!("List: {:?}", list);
    let (start, end) = list.extract().ok_or(ServerFnError::new("Bad date"))?;
    let FromContext(state): FromContext<AppState> = extract().await?;
    state
        .list_eventsf::<format::Tana>(start, end)
        .await
        .map_err(ServerFnError::new)
}
