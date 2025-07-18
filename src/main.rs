mod args;
use args::Args;
use clap::Parser;
use log::{debug};
mod fs_parser;
use fs_parser::{parse_fs};
use std::path::PathBuf;
mod visualize;
use visualize::visualize;

fn main() {
    env_logger::init();

    let args = Args::parse();
    debug!("Parsed Args: {:#?}", args);

    let filestructure = parse_fs(PathBuf::from(args.input_path), args.depth);
    debug!("Parsed filestructure: {:#?}", filestructure);

    visualize(filestructure, args.theme, args.filetype, PathBuf::from(args.output_filepath));
}
