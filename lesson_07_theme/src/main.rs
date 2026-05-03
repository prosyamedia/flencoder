use fltk::{prelude::*, *, enums::Color};
use crate::flencoder_ui::UserInterface;
use fltk_theme::{widget_themes, WidgetTheme, ThemeType};

mod flencoder_ui {
    fl2rust_macro::include_ui!("src/ui.fl");
}

fn main() {
    let app = app::App::default();
    
    WidgetTheme::new(ThemeType::Dark).apply();

    // Assuming 'UserInterface' is a struct you've defined 
    // or generated (e.g., using fl2rust)
    let mut ui = UserInterface::init();

    // Set main window resizable to invisible group that covers all other groups
    //ui.main_window.resizable(&ui.resize_group);
    let min_w = ui.main_window.w();
    let min_h = ui.main_window.h();

    ui.main_window.resize_callback(move |win, x, y, w_new, h_new| {
        let mut w = w_new;
        let mut h = h_new;

        if w_new < min_w {
            w = min_w;
        }
        if h_new < min_h {
            h = min_h;
        }

        // Apply corrected size
        // Only resize if we are correcting the size
        if w != w_new || h != h_new {
            win.resize(x, y, w, h);
        }
    });


    ui.main_window.show();
    let _=app.run();
}
