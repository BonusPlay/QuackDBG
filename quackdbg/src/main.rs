use color_eyre::eyre::Result;
use std::sync::{Arc, Mutex};
use log::info;

mod debugger;
mod gui;
mod context;
use context::AppContext;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<()> {
    color_eyre::install()?;
    env_logger::init();

    info!("Running QuackDBG {}", VERSION);

    let ctx = Arc::new(Mutex::new(AppContext::new()?));
    gui::create(ctx);

    Ok(())
}
