mod calendar;
pub use calendar::*;
pub mod db;
pub use db::Db;

pub mod state;
pub use state::State;

pub mod format;
pub mod types;

pub fn ip_port() -> (String, u16) {
    let ip = std::env::var("SHADOCAL_IP");
    let port = std::env::var("SHADOCAL_PORT").map(|v| {
        v.parse::<u16>()
            .expect("Invalid environment variable: SHADOCAL_PORT. Must be a valid port number.")
    });
    (ip.unwrap_or("127.0.0.1".to_string()), port.unwrap_or(7117))
}
