pub mod hello;

mod console;
pub use console::Console;

pub trait Widget {
    fn init(&mut self, ui: &imgui::Ui);
}
