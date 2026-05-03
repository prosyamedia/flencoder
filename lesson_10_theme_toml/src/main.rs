use fltk::{prelude::*, *, enums::Color, button::Button, app::Scheme};
use crate::flencoder_ui::UserInterface;
use fltk_theme::{WidgetTheme, ThemeType};
use fltk::image::SvgImage;
use serde::Deserialize;

#[derive(Deserialize)]
struct WidgetScheme {
    scheme: String,
}

#[derive(Deserialize)]
struct ColorTheme {
    theme: String,
}

#[derive(Deserialize)]
struct ThemePreset {
    widget_scheme: WidgetScheme,
    color_theme: ColorTheme,
}

mod flencoder_ui {
    fl2rust_macro::include_ui!("src/ui.fl");
}

fn theme_from_str(name: &str) -> ThemeType {
    match name.to_lowercase().as_str() {
        "dark" => ThemeType::Dark,
        "blue" => ThemeType::Blue,
        "aero" => ThemeType::Aero,
        "classic" => ThemeType::Classic,
        "greybird" => ThemeType::Greybird,
        "highcontrast" => ThemeType::HighContrast,
        "metro" => ThemeType::Metro,
        _ => ThemeType::Dark, // fallback
    }
}

fn scheme_from_str(s: &str) -> Option<Scheme> {
    match s.to_lowercase().as_str() {
        "base" => Some(Scheme::Base),
        "gtk+" => Some(Scheme::Gtk),
        "plastic" => Some(Scheme::Plastic),
        "gleam" => Some(Scheme::Gleam),
        "oxy" => Some(Scheme::Oxy),
        "none" => None, // means: do not set a scheme
        _ => None,
    }
}

fn apply_theme_preset(path: &str) {
    let toml_str = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("❌ Failed to read theme preset '{}': {}", path, e);
            return; // or exit(1)
        }
    };

    let preset: ThemePreset = match toml::from_str(&toml_str) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("❌ Failed to parse theme preset '{}': {}", path, e);
            return;
        }
    };

    // 1. FLTK widget scheme
    if let Some(scheme) = scheme_from_str(&preset.widget_scheme.scheme) {
        fltk::app::set_scheme(scheme);
    }

    // 2. fltk-theme widget theme
    let theme_type = theme_from_str(&preset.color_theme.theme);
    WidgetTheme::new(theme_type).apply();
}

fn main() {
    let app = app::App::default();
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

    let svg_icon = SvgImage::from_data(svg_str).expect("invalid svg");

    // Assuming 'UserInterface' is a struct you've defined 
    // or generated (e.g., using fl2rust)
    let mut ui = UserInterface::init();
    // Set as window icon
    ui.main_window.set_icon(Some(svg_icon));

    // Read in scheme and theme from toml
    apply_theme_preset("assets/toml/theme.toml");
    //WidgetTheme::new(ThemeType::Dark).apply();

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
                println!("set {:?} for {}", Color::Selection.to_rgb(), this.label());
                this.set_color(Color::Background2);
                this.redraw();
                app::redraw();
            }
        });
    };

    ui.main_window.show();

    let mut svg_logo = SvgImage::from_data(svg_str).expect("invalid svg");
    // Set logo in top_group
    // THEN scale down to fit the box
    svg_logo.scale(ui.logo_box.w(), ui.logo_box.h(), true, true);

    // Attach
    ui.logo_box.set_label("");
    ui.logo_box.set_image(Some(svg_logo));
    ui.logo_box.redraw();    
    
    let _=app.run();
}
