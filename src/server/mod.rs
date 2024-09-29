use actix_cors::Cors;
use actix_web::{get, http, web, App, HttpResponse, HttpServer, Responder};

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
    pub async fn run(self, config: Option<Config>) -> anyhow::Result<()> {
        let state = web::Data::new(State::new(config).await?);

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

pub fn ip_port() -> (String, u16) {
    let ip = std::env::var("SHADOCAL_IP");
    let port = std::env::var("SHADOCAL_PORT").map(|v| {
        v.parse::<u16>()
            .expect("Invalid environment variable: SHADOCAL_PORT. Must be a valid port number.")
    });
    (ip.unwrap_or("127.0.0.1".to_string()), port.unwrap_or(7117))
}
