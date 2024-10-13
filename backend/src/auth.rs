use actix_web::{
    get,
    http::header,
    web::{Data, Query},
    HttpResponse,
};
use shadocal_lib::{CalendarType, OAuthRequest, State, OAUTH};

#[get("/login")]
async fn login() -> HttpResponse {
    let auth_url = OAUTH.auth_url().await;
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url))
        .finish()
}

#[get("/auth")]
async fn auth(data: Data<State>, Query(params): Query<OAuthRequest>) -> HttpResponse {
    let (_, token) = OAUTH.auth(params).await.expect("Failed to get token");
    data.into_inner()
        .new_calendar(CalendarType::Google, String::new(), Some(token))
        .await
        .expect("Failed to add new calendar");

    HttpResponse::Ok().body(format!(
        r#"<html>
        <head><title>Authorized</title></head>
        <body>
            Go back to the app!
        </body>
    </html>"#,
    ))
}
