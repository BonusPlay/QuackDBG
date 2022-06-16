use std::sync::{Arc, RwLock};
use crossbeam::channel::Sender;

use crate::context::Context;
use crate::gui::colors::*;
use crate::gui::widgets::Widget;

pub struct Console {
    cur_input: String,
    log: Arc<RwLock<Vec<String>>>,
    dbg_tx: Sender<String>,
}

impl Console {
    pub fn new(ctx: &Context) -> Self {
        let this = Self {
            cur_input: String::with_capacity(1024),
            log: Arc::new(RwLock::new(vec![])),
            dbg_tx: ctx.lock().unwrap().debugger.add_transmitter(),
        };

        let debugger = ctx.lock().unwrap().debugger.add_listener();
        let writer = this.log.clone();

        std::thread::spawn(move || {
            for line in debugger.iter() {
                (*writer.write().unwrap()).push(line);
            }
        });

        return this
    }
}

impl Widget for Console {
    fn init(&mut self, ui: &imgui::Ui) {
        use imgui::*;

        Window::new("Debugger console")
            .size([600.0, 200.0], Condition::FirstUseEver)
            .build(ui, || {
                for line in self.log.read().unwrap().iter() {
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
                    self.dbg_tx.send(self.cur_input.to_owned()).unwrap();
                    (*self.log.write().unwrap()).push(self.cur_input.to_owned());
                    self.cur_input = String::with_capacity(1024);
                }
            });
    }
}
