use actix_web::{
    get,
    http::header,
    web::{self, Data, Query},
    HttpResponse,
};
use shadocal_lib::{CalendarType, OAuthRequest, OAUTH};

use super::BState;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(login).service(auth);
}

#[get("/login")]
async fn login() -> HttpResponse {
    let auth_url = OAUTH.auth_url().await;
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url))
        .finish()
}

#[get("/auth")]
async fn auth(data: Data<BState>, Query(params): Query<OAuthRequest>) -> HttpResponse {
    let (_, token) = OAUTH.auth(params).await.expect("Failed to get token");
    data.write()
        .await
        .new_calendar(CalendarType::Google, String::new(), Some(token))
        .await
        .expect("Failed to add new calendar");

    HttpResponse::Ok().body(
        r#"<html>
        <head><title>Authorized</title></head>
        <body>
            Go back to the app!
        </body>
    </html>"#,
    )
}
