use actix_cors::Cors;
use actix_web::{get, http, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

use shadocal_lib::{Db, State};

use super::{account, tana};

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
    pub async fn run(self, db: Db) -> anyhow::Result<()> {
        let state = web::Data::new(State::new(db).await?);

        println!("🚀 Server started successfully");
        HttpServer::new(move || {
            App::new()
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allowed_origin("https://app.tana.inc")
                        .allowed_methods(vec!["GET", "POST"])
                        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                        .allowed_header(http::header::CONTENT_TYPE),
                )
                .service(index)
                .configure(tana::config)
                .configure(account::config)
                .app_data(state.clone())
        })
        .bind((self.ip, self.port))?
        .run()
        .await?;
        Ok(())
    }
}
