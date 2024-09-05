use actix_cors::Cors;
use actix_web::{error, get, web, App, HttpResponse, HttpServer, Responder};

use super::*;

mod state;
use state::*;
mod tana;

pub struct Server {
    ip: String,
    port: u16,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/raw")]
async fn raw(_: web::Data<State>) -> actix_web::Result<String> {
    // data.get_events::<format::Raw>()
    //     .await
    //     .map_err(|err| error::ErrorFailedDependency(err.to_string()))
    todo!()
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
    pub async fn run(self, cal: Calendar) -> anyhow::Result<()> {
        let state = web::Data::new(State::new(cal));

        println!("🚀 Server started successfully");
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .wrap(
                    Cors::default()
                        .allowed_origin("https://app.tana.inc")
                        .allowed_methods(vec!["GET", "POST"]),
                )
                .service(index)
                .configure(tana::config)
        })
        .bind((self.ip, self.port))?
        .run()
        .await?;
        Ok(())
    }
}
