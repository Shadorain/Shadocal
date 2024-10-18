use actix_web::{
    get,
    http::header,
    web::{self, Data, Query},
    HttpResponse,
};
use shadocal_lib::{CalendarType, OAuthRequest, State, OAUTH};

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(web::scope("/auth").service(login).service(authenticate));
}

#[get("/login")]
async fn login() -> HttpResponse {
    let auth_url = OAUTH.auth_url().await;
    HttpResponse::Found()
        .append_header((header::LOCATION, auth_url))
        .finish()
}

#[get("/authenticate")]
async fn authenticate(data: Data<State>, Query(params): Query<OAuthRequest>) -> HttpResponse {
    let Ok((_, token)) = OAUTH.auth(params).await else {
        return HttpResponse::InternalServerError().into();
    };
    data.new_calendar(CalendarType::Google, Some(token))
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
