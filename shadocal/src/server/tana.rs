use dioxus::prelude::*;
use server_fn::codec::Json;

#[cfg(feature = "server")]
use shadocal_lib::format;
use shadocal_lib::types::{Get, List};

#[server(
  name = TanaGet,
  prefix = "/tana",
  endpoint = "get",
  input = Json,
)]
pub async fn get(get: Get) -> Result<String, ServerFnError> {
    println!("Get: {:?}", get);
    crate::SHADOCAL
        .read()
        .await
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
    crate::SHADOCAL
        .read()
        .await
        .list_eventsf::<format::Tana>(start, end)
        .await
        .map_err(ServerFnError::new)
}
