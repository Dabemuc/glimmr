use crate::args::filetypes::Filetype;
use crate::args::themes::Theme;
use crate::fs_parser::fs_structs::FlatFsEntry;
use std::path::PathBuf;
mod svg_helper;
use log::debug;
use svg_helper::compose_svg_from_filestruct;

pub fn visualize(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    filetype: Filetype,
    output_filepath: PathBuf,
    bake_font: bool,
) {
    match filetype {
        Filetype::SVG => build_svg(
            filestructure,
            theme,
            output_filepath,
            Filetype::SVG.extension(),
            bake_font,
        ),
        Filetype::PNG => build_png(
            filestructure,
            theme,
            output_filepath,
            Filetype::PNG.extension(),
        ),
    }
}

fn build_svg(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    mut output_filepath: PathBuf,
    extension: &'static str,
    bake_font: bool,
) {
    // Compose svg
    let document = compose_svg_from_filestruct(filestructure, theme, bake_font);

    // Output
    debug!("Provided output_filepath: {}", output_filepath.display());
    if output_filepath.extension().is_none() {
        output_filepath.set_extension(extension);
        debug!("After adding extension: {}", output_filepath.display());
    }
    svg::save(output_filepath, &document).unwrap();
}

fn build_png(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    mut output_filepath: PathBuf,
    extension: &'static str,
) {
    // Compose svg, always bake font for PNG rendering
    let document = compose_svg_from_filestruct(filestructure, theme, true);
    let svg_data = document.to_string();

    // Create a font database and load system fonts
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_system_fonts();

    // Parse SVG with usvg
    let tree = usvg::Tree::from_data(svg_data.as_bytes(), &usvg::Options::default(), &fontdb).unwrap();

    // Render with resvg
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    
    resvg::render(&tree, resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // Save PNG
    debug!("Provided output_filepath: {}", output_filepath.display());
    if output_filepath.extension().is_none() {
        output_filepath.set_extension(extension);
        debug!("After adding extension: {}", output_filepath.display());
    }
    pixmap.save_png(output_filepath).unwrap();
}
