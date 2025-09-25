use base64::{Engine, engine::general_purpose};
use font_kit::handle::Handle;
use font_kit::source::SystemSource;
use log::debug;
use rusttype::{Font, Scale};
use std::fs;

pub fn load_font_bytes(font_name: &str) -> Option<Vec<u8>> {
    // Check embedded fonts first
    let embedded_font: Option<&'static [u8]> = match font_name {
        "Arial" => Some(include_bytes!("../../../assets/fonts/ARIAL.TTF")),
        "Fira Code" => Some(include_bytes!(
            "../../../assets/fonts/FiraCode-VariableFont_wght.ttf"
        )),
        "Menlo" => Some(include_bytes!("../../../assets/fonts/Menlo-Regular.ttf")),
        "Ubuntu Mono" => Some(include_bytes!(
            "../../../assets/fonts/UbuntuMono-Regular.ttf"
        )),
        "Consolas" => Some(include_bytes!("../../../assets/fonts/consolas.ttf")),
        "Courier New" => Some(include_bytes!("../../../assets/fonts/cour.ttf")),
        _ => None,
    };

    if let Some(bytes) = embedded_font {
        debug!(
            "Font '{}' loaded successfully from embedded assets.",
            font_name
        );
        return Some(bytes.to_vec());
    }

    // Check system fonts
    let source = SystemSource::new();
    if let Ok(handle) = source.select_by_postscript_name(font_name) {
        match handle {
            Handle::Path { path, .. } => {
                debug!(
                    "Font '{}' loaded successfully from system path: {:?}",
                    font_name, path
                );
                return fs::read(path).ok();
            }
            Handle::Memory { bytes, .. } => {
                debug!(
                    "Font '{}' loaded successfully from system memory.",
                    font_name
                );
                return Some(bytes.to_vec());
            }
        }
    }

    // Not found
    debug!(
        "Font '{}' could not be found in embedded assets or system fonts.",
        font_name
    );
    None
}

pub fn measure_text_width(text: &str, font: &Font, font_size: f32) -> f32 {
    let scale = Scale::uniform(font_size);
    let start = rusttype::point(0.0, 0.0);

    let mut last_x: f32 = 0.0;
    let mut max_x: f32 = 0.0;

    for g in font.layout(text, scale, start) {
        // The pen position for this glyph
        let pos_x = g.position().x;

        // Update max_x to include bounding box right edge if available
        if let Some(bb) = g.pixel_bounding_box() {
            max_x = f32::max(max_x, bb.max.x as f32);
        }

        // Track the rightmost pen position as well
        last_x = pos_x + g.unpositioned().h_metrics().advance_width;
    }

    // Real width is either from pen advance or bounding box overhang
    f32::max(last_x, max_x)
}

pub fn build_b64_font_embed(font_bytes: &[u8], font_family: &str) -> String {
    let encoded = general_purpose::STANDARD.encode(font_bytes);
    format!(
        "<style>@font-face {{
            font-family: '{}';
            src: url('data:font/ttf;base64,{}') format('truetype');
        }}</style>",
        font_family, encoded
    )
}
