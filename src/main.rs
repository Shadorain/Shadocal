use anyhow::Result;

mod calendar;
use calendar::*;
mod format;
use format::Format;
mod server;
use server::Server;
mod config;
use config::Config;

#[actix_web::main]
async fn main() -> Result<()> {
    let (ip, port) = server::ip_port();
    let config = Config::new(None)?;
    Server::new(ip, port).run(Some(config)).await
}
