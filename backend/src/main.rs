use anyhow::Result;

use shadocal_lib::{ip_port, State};

mod server;
use server::*;
mod auth;
mod config;
mod tana;
use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let (ip, port) = ip_port();
    Server::new(ip, port)
        .run(State::new().configure(Config::new(None)?.calendars).await?)
        .await
}
