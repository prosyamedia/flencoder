use fltk::{prelude::*, *, enums::Color};
use crate::flencoder_ui::UserInterface;

mod flencoder_ui {
    fl2rust_macro::include_ui!("src/ui.fl");
}

fn main() {
    let app = app::App::default();
    
    // Assuming 'UserInterface' is a struct you've defined 
    // or generated (e.g., using fl2rust)
    let mut ui = UserInterface::init();

    // Set main window resizable to invisible group that covers all other groups
    ui.main_window.resizable(&ui.resize_group);
    ui.main_window.show();
    let _=app.run();
}
