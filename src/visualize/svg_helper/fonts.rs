use font_kit::source::SystemSource;
use font_kit::handle::Handle;
use rusttype::{Font, Scale};
use std::fs;
use base64::{engine::general_purpose, Engine};

pub fn load_font_bytes(font_name: &str) -> Option<Vec<u8>> {
    let source = SystemSource::new();
    let handle = source.select_by_postscript_name(font_name).ok()?;

    match handle {
        Handle::Path { path, .. } => fs::read(path).ok(),
        Handle::Memory { bytes, .. } => Some(bytes.to_vec()),
    }
}

pub fn measure_text_width(text: &str, font: &Font, font_size: f32) -> f32 {
    let scale = Scale::uniform(font_size);
    font.layout(text, scale, rusttype::point(0.0, 0.0))
        .map(|g| g.unpositioned().h_metrics().advance_width)
        .sum()
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
