mod support;
mod main_window;

mod widgets;
use widgets::Widget;
use widgets::console::Console;

pub fn create() {
    let system = support::init(file!());
    let mut console: Console = Default::default();

    system.main_loop(move |run, ui| {
        main_window::init(&ui);
        widgets::hello::init(&ui);
        console.init(&ui);
        ui.show_demo_window(run);
    });
}
