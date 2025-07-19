use crate::args::themes::Theme;
use crate::args::filetypes::Filetype;
use crate::fs_parser::fs_structs::Folder;
use std::path::PathBuf;
mod svg_helper;
use log::debug;
use svg_helper::compose_svg_from_filestruct;

pub fn visualize(filestructure: Folder, theme: Theme, filetype: Filetype, output_filepath: PathBuf, include_root: bool) {
    match filetype {
        Filetype::SVG => build_svg(filestructure, theme, output_filepath, include_root, Filetype::SVG.extension()),
        Filetype::PNG => build_png(filestructure, theme, output_filepath, include_root, Filetype::PNG.extension()),
    }
}

fn build_svg(filestructure: Folder, theme: Theme, mut output_filepath: PathBuf, include_root: bool, extension: &'static str) {
    // Compose svg
    let document = compose_svg_from_filestruct(filestructure, theme, include_root);

    // Output
    debug!("Provided output_filepath: {}", output_filepath.display());
    if output_filepath.extension().is_none() {
        output_filepath.set_extension(extension);
        debug!("After adding extension: {}", output_filepath.display());
    }
    svg::save(output_filepath, &document).unwrap();
}

fn build_png(_filestructure: Folder, _theme: Theme, _output_filepath: PathBuf, _include_root: bool, _extension: &'static str) {
    panic!("PNG not yet implemented!");
}
