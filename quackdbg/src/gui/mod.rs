pub mod colors;

mod support;
mod main_window;
mod widgets;

use widgets::Widget;
use crate::context::Context;

pub fn create(ctx: Context) {
    let system = support::init();
    let mut console = widgets::Console::new(&ctx);
    ctx.lock().unwrap().debugger.start();

    system.main_loop(move |run, ui| {
        main_window::init(&ui);
        widgets::hello::init(&ui);
        console.init(&ui);
        ui.show_demo_window(run);
    });
}
