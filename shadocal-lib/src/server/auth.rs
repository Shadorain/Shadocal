use actix_web::{
    get,
    http::header,
    web::{Data, Query},
    HttpResponse,
};
use gcal_rs::OAuthRequest;

use super::State;

// #[get("/login")]
// async fn login(data: Data<State>) -> HttpResponse {
//     let auth_url = data.oauth.lock().await.auth_url();
//     HttpResponse::Found()
//         .append_header((header::LOCATION, auth_url.to_string()))
//         .finish()
// }
//
// #[get("/auth")]
// async fn auth(data: Data<State>, Query(params): Query<AuthRequest>) -> HttpResponse {
//     let (state, token) = data
//         .oauth
//         .lock()
//         .await
//         .auth(params)
//         .await
//         .expect("Failed to get token");
//
//     let html = format!(
//         r#"<html>
//         <head><title>OAuth2 Test</title></head>
//         <body>
//             Google returned the following state:
//             <pre>{}</pre>
//             Google returned the following token:
//             <pre>{:?}</pre>
//         </body>
//     </html>"#,
//         state.secret(),
//         token
//     );
//     HttpResponse::Ok().body(html)
// }
