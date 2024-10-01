#![allow(non_snake_case)]
use anyhow::Result;

mod app;
use app::App;

fn main() -> Result<()> {
    App::run(None)
}
