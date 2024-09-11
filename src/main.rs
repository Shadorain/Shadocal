mod calendar;
use calendar::*;
mod format;
use format::Format;
mod server;
use server::Server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let ip = std::env::var("SHADOGCAL_IP");
    let port = std::env::var("SHADOGCAL_PORT").map(|v| {
        v.parse::<u16>()
            .expect("Invalid environment variable: SHADOGCAL_PORT. Must be a valid port number.")
    });

    let cal = Calendar::new().await.expect("Failed to create calendar");
    Server::new(ip.unwrap_or("127.0.0.1".to_string()), port.unwrap_or(7117))
        .run(cal)
        .await
}
