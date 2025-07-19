use crate::args::themes::Theme;
use crate::args::filetypes::Filetype;
use crate::fs_parser::fs_structs::Folder;
use std::path::PathBuf;
mod svg_helper;
use log::debug;
use svg_helper::compose_svg_from_filestruct;

pub fn visualize(filestructure: Folder, theme: Theme, filetype: Filetype, output_filepath: PathBuf, include_root: bool, width: Option<u32>, heigth: Option<u32>) {
    match filetype {
        Filetype::SVG => build_svg(filestructure, theme, output_filepath, include_root, Filetype::SVG.extension(), width, heigth),
        Filetype::PNG => build_png(filestructure, theme, output_filepath, include_root, Filetype::PNG.extension(), width, heigth),
    }
}

fn build_svg(filestructure: Folder, theme: Theme, mut output_filepath: PathBuf, include_root: bool, extension: &'static str, width: Option<u32>, heigth: Option<u32>) {
    // Compose svg
    let document = compose_svg_from_filestruct(filestructure, theme, include_root, width, heigth);

    // Output
    debug!("Provided output_filepath: {}", output_filepath.display());
    if output_filepath.extension().is_none() {
        output_filepath.set_extension(extension);
        debug!("After adding extension: {}", output_filepath.display());
    }
    svg::save(output_filepath, &document).unwrap();
}

fn build_png(_filestructure: Folder, _theme: Theme, _output_filepath: PathBuf, _include_root: bool, _extension: &'static str, width: Option<u32>, heigth: Option<u32>) {
    panic!("PNG not yet implemented!");
}
