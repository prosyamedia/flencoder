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
    ui.main_window.resizable(&ui.resize_group);
    ui.main_window.show();
    let _=app.run();
}
