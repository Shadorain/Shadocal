use anyhow::Result;

use shadocal_lib::{server, Config, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let (ip, port) = server::ip_port();
    let config = Config::new(None)?;
    Server::new(ip, port).run(Some(config)).await
}
