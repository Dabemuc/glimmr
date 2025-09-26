mod args;
use args::Args;
use clap::Parser;
use log::debug;
mod fs_parser;
use fs_parser::parse_fs_flat;
use std::path::PathBuf;
mod visualize;
use visualize::visualize;

fn main() {
    env_logger::init();

    let args = Args::parse();
    debug!("Parsed Args: {:#?}", args);

    let filestructure = parse_fs_flat(
        args.input_path,
        args.depth,
        args.include_root,
        args.excludes,
    );
    debug!("Parsed filestructure: {:#?}", filestructure);

    visualize(
        filestructure,
        args.theme.get_theme(),
        args.filetype,
        PathBuf::from(args.output_filepath),
        args.bake_font,
    );
}
