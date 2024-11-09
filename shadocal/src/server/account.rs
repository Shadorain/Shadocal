use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::AppState;
use shadocal_lib::Profile;

#[server(name = AccountList, prefix = "/account", endpoint = "list")]
pub async fn account_list() -> Result<Vec<Profile>, ServerFnError> {
    let FromContext(state): FromContext<AppState> = extract().await?;
    state.db.list_accounts().map_err(ServerFnError::new)
}
