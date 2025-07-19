use crate::args::themes::Theme;
use crate::args::filetypes::Filetype;
use crate::fs_parser::fs_structs::Folder;
use std::path::PathBuf;
mod svg_helper;
use svg_helper::compose_svg_from_filestruct;

pub fn visualize(filestructure: Folder, theme: Theme, filetype: Filetype, output_filepath: PathBuf, include_root: bool) {
    match filetype {
        Filetype::SVG => build_svg(filestructure, theme, output_filepath, include_root),
        Filetype::PNG => build_png(filestructure, theme, output_filepath, include_root),
    }
}

fn build_svg(filestructure: Folder, theme: Theme, _output_filepath: PathBuf, include_root: bool) {
    // Compose svg
    let document = compose_svg_from_filestruct(filestructure, theme, include_root);

    // Output
    // TODO: Use output_filepath, check for fileextension and set if not provided
    svg::save("glimmr_out.svg", &document).unwrap();
}

fn build_png(_filestructure: Folder, _theme: Theme, _output_filepath: PathBuf, _include_root: bool) {
    panic!("PNG not yet implemented!");
}
