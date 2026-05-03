use fltk::{prelude::*, *, enums::Color, button::Button};
use crate::flencoder_ui::UserInterface;
use fltk_theme::{WidgetTheme, ThemeType};
use fltk::image::SvgImage;



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

    

    // Set color for all groups.
    ui.left_group.set_color(Color::Inactive);
    ui.middle_group.set_color(Color::Inactive);
    ui.right_group.set_color(Color::Inactive);

    let mut codec_classes:Vec<Button> = vec! [
        ui.intermediate_button.clone(),
        ui.delivery_button.clone(),
        ui.master_button.clone(),
        ui.archive_button.clone(),
        ui.streaming_button.clone(),
        ui.web_optimized_button.clone(),
        ui.quick_presets_button.clone()];
        
    // set selection highlight for all Codec Classes
    for i in 0..codec_classes.len() {
        let cc = codec_classes.clone();  // clone the vec for this callback
        let mut this = codec_classes[i].clone();  // ← clone this specific button
        codec_classes[i].set_callback ({
            move |_| {
                for mut btn in cc.clone() {
                    btn.set_color(Color::Background);
                };
                println!("set {} for {}", Color::Selection, this.label());
                this.set_color(Color::Background2);
                this.redraw();
                app::redraw();
            }
        });
    };

    ui.main_window.show();
    // Set icon in top_group
    // Load SVG from string
    let svg_str = r##"
<svg width="200" height="200" viewBox="0 0 100 100"
     xmlns="http://www.w3.org/2000/svg">
  <rect x="0" y="0" width="100" height="100" fill="#1E90FF"/>
  <path d="
    M 30 20
    H 70
    V 32
    H 42
    V 44
    H 65
    V 56
    H 42
    V 80
    H 30
    Z
  " fill="#FFFFFF"/>
</svg>
"##;

    let mut svg = SvgImage::from_data(svg_str).expect("invalid svg");

    // THEN scale down to fit the box
    svg.scale(ui.logo_box.w(), ui.logo_box.h(), true, true);
    println!("Scale to {}x{}", ui.logo_box.w(), ui.logo_box.h());

    // Attach
    ui.logo_box.set_label("");
    ui.logo_box.set_image(Some(svg));
    ui.logo_box.redraw();    
    
    let _=app.run();
}
