use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer};
use actix_web::{HttpResponse, Responder};
use anyhow::Result;

mod calendar;
use calendar::Calendar;
mod format;
use format::Format;

struct AppState {
    cal: Calendar,
}
impl AppState {
    pub fn new() -> Result<Self> {
        Ok(Self {
            cal: Calendar::new()?,
        })
    }

    pub async fn process_events(&self, format: Format) -> Result<String> {
        let now = chrono::Local::now();
        let events = self
            .cal
            .get_events(now - chrono::Duration::days(1), now)
            .await?;

        for event in &events {
            eprintln!("{:?} {:?}", event.id, event.summary);
        }
        format
            .format(events)
            .ok_or(anyhow::anyhow!("No events found".to_string()))
    }
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/raw")]
async fn raw(data: web::Data<AppState>) -> actix_web::Result<String> {
    data.process_events(Format::Raw)
        .await
        .map_err(|err| actix_web::error::ErrorFailedDependency(err.to_string()))
}

#[get("/tana")]
async fn tana(data: web::Data<AppState>) -> actix_web::Result<String> {
    data.process_events(Format::Tana)
        .await
        .map_err(|err| actix_web::error::ErrorFailedDependency(err.to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    println!("🚀 Server started successfully");
    // std::env::var("CALENDER_ACCESS_FILE").expect("Provide an access key"),
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(
                AppState::new().expect("Failed to create app state"),
            ))
            .wrap(
                Cors::default()
                    .allowed_origin("https://app.tana.inc")
                    .allowed_methods(vec!["GET", "POST"]),
            )
            .service(index)
            .service(tana)
    })
    .bind(("127.0.0.1", 3001))?
    .run()
    .await
}
