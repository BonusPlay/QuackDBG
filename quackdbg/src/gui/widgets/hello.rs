use imgui::*;

pub fn init(ui: &Ui) {
    Window::new("Hello world")
        .size([300.0, 110.0], Condition::FirstUseEver)
        .build(ui, || {
            ui.text_wrapped("Hello world!");
            ui.separator();
            ui.button("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
}
