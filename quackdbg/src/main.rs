use color_eyre::eyre::Result;
use std::sync::{Arc, Mutex};

mod debugger;
mod gui;

mod context;
use context::AppContext;

fn main() -> Result<()> {
    color_eyre::install()?;

    let ctx = Arc::new(Mutex::new(AppContext::new()?));
    gui::create(ctx);

    Ok(())
}
