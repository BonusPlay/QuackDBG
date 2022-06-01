use imgui::*;

use crate::gui::colors::*;
use crate::gui::widgets::Widget;

pub struct Console {
    cur_input: String,
    log: Vec<String>,
}

impl Default for Console {
    fn default() -> Self {
        Self {
            cur_input: String::with_capacity(1024),
            log: vec![]
        }
    }
}

impl Widget for Console {
    fn init(&mut self, ui: &Ui) {
        Window::new("Debugger console")
            .size([300.0, 110.0], Condition::FirstUseEver)
            .build(ui, || {
                for line in self.log.iter() {
                    let color = match line.split(" ").next().unwrap() {
                        "[error]" => RED,
                        "[warning]" => YELLOW,
                        "[success]" => GREEN,
                        _ => WHITE,
                    };

                    ui.text_colored(color, line);
                }

                let enter = ui.input_text("", &mut self.cur_input)
                    .flags(InputTextFlags::ENTER_RETURNS_TRUE)
                    .build();

                if enter {
                    self.log.push(self.cur_input.to_owned());
                    self.cur_input = String::with_capacity(1024);
                }
            });
    }
}
