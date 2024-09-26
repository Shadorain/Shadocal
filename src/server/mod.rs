use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};

mod auth;
mod state;
mod tana;
mod types;

use state::*;
use types::*;

use super::*;

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
    todo!()
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }
    pub async fn run(self) -> anyhow::Result<()> {
        let state = web::Data::new(State::new());

        println!("🚀 Server started successfully");
        HttpServer::new(move || {
            App::new()
                .app_data(state.clone())
                .wrap(
                    Cors::default()
                        .allowed_origin("https://app.tana.inc")
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE),
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
