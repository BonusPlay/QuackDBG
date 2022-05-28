pub mod hello;
pub mod console;

pub trait Widget {
    fn init(&mut self, ui: &imgui::Ui);
}
