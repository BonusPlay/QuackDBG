use imgui::{MenuItem, Ui, Window};

pub fn init(ui: &Ui) {
    Window::new("QuackDBG")
        .no_decoration()
        .draw_background(true)
        .build(ui, || {
            if let Some(menu_bar) = ui.begin_main_menu_bar() {
                if let Some(menu) = ui.begin_menu("Options") {
                    MenuItem::new("TODO").build(ui);
                    menu.end();
                }
                menu_bar.end();
            }
        });
}
