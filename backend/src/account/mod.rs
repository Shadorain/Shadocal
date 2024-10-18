use actix_web::web;

mod auth;
mod operations;

pub fn config(conf: &mut web::ServiceConfig) {
    conf.service(
        web::scope("/account")
            .configure(auth::config)
            .configure(operations::config),
    );
}
