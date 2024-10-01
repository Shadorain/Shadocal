use anyhow::Result;

use shadocal_lib::{ip_port, State};

mod server;
use server::*;
mod tana;

#[tokio::main]
async fn main() -> Result<()> {
    let (ip, port) = ip_port();
    let state = State::new(None).await?;
    Server::new(ip, port).run(state).await
}
