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
    _filestructure: Vec<FlatFsEntry>,
    _theme: Theme,
    _output_filepath: PathBuf,
    _extension: &'static str,
) {
    panic!("PNG not yet implemented!");
}
