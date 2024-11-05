use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
};
use dioxus::prelude::*;
use shadocal_lib::{CalendarType, OAuthRequest, OAUTH};

use super::AppState;

pub async fn login() -> Redirect {
    // println!("Loginng in...");
    Redirect::temporary(&OAUTH.auth_url().await)
}

pub async fn authenticate(
    State(state): State<AppState>,
    Query(params): Query<OAuthRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    // println!("Auth: {:?}", params);

    let (_, token) = OAUTH
        .auth(params)
        .await
        .map_err(|_| StatusCode::NOT_ACCEPTABLE)?;
    state
        .new_calendar(CalendarType::Google, Some(token))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Html(
        r#"<html>
        <head><title>Authorized</title></head>
        <body>
            Successfully authenticated. Return to your application! You can close this window.
        </body>
    </html>"#,
    ))
}
