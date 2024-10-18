mod server;
use server::*;
mod account;
mod tana;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // init_logger()?;

    let (ip, port) = shadocal_lib::ip_port();
    Server::new(ip, port)
        .run(shadocal_lib::Db::new(None)?)
        .await
}

// pub fn init_logger() -> anyhow::Result<()> {
//     use log::{Level, LevelFilter, Metadata, Record};
//
//     std::panic::set_hook(Box::new(|panic_info| {
//         log::error!("Panic occurred: {}", panic_info);
//         std::process::exit(1);
//     }));
//
//     struct SimpleLogger;
//     impl log::Log for SimpleLogger {
//         fn enabled(&self, metadata: &Metadata) -> bool {
//             metadata.level() <= Level::Info
//         }
//
//         fn log(&self, record: &Record) {
//             if self.enabled(record.metadata()) {
//                 println!("{} - {}", record.level(), record.args());
//             }
//         }
//
//         fn flush(&self) {}
//     }
//
//     static LOGGER: SimpleLogger = SimpleLogger;
//
//     log::set_logger(&LOGGER)
//         .map(|()| log::set_max_level(LevelFilter::Info))
//         .expect("Failed to set logger");
//     Ok(())
// }
