mod calendar;
use calendar::*;
mod format;
use format::Format;
mod server;
use server::Server;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let cal = Calendar::new().await.expect("Failed to create calendar");
    Server::new("127.0.0.1".to_string(), 7117).run(cal).await
}
