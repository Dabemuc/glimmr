use crate::args::themes::Theme;
use crate::args::filetypes::Filetype;
use crate::fs_parser::fs_structs::Folder;
use std::path::PathBuf;

pub fn visualize(filestructure: Folder, theme: Theme, filetype: Filetype, output_filepath: PathBuf) {
    match filetype {
        Filetype::SVG => build_svg(filestructure, theme, output_filepath),
        Filetype::PNG => build_png(filestructure, theme, output_filepath),
    }
}

fn build_svg(_filestructure: Folder, _theme: Theme, _output_filepath: PathBuf) {
    panic!("SVG not yet implemented!");
}

fn build_png(_filestructure: Folder, _theme: Theme, _output_filepath: PathBuf) {
    panic!("PNG not yet implemented!");
}
