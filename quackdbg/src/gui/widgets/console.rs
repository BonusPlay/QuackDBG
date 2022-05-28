use imgui::*;

use crate::gui::widgets::Widget;

#[derive(Default)]
pub struct Console {
    cur_input: String,
}

impl Widget for Console {
    fn init(&mut self, ui: &Ui) {
        Window::new("Debugger console")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.input_text("input text with hint", &mut self.cur_input)
                    .hint("raw debugger comand")
                    .build();
            });
    }
}
