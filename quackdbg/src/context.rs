use color_eyre::eyre::Result;
use std::sync::{Arc, Mutex};

use crate::debugger::gdb::GDB;

pub struct AppContext {
    pub debugger: GDB,
}


impl AppContext {
    pub fn new() -> Result<Self> {
        let debugger = GDB::new()?;

        Ok(Self {
            debugger,
        })
    }
}

pub type Context = Arc<Mutex<AppContext>>;
