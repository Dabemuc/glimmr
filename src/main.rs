mod args;
use args::Args;
use clap::Parser;
use log::{debug};
mod fs_parser;
use fs_parser::{parse_fs};
use std::path::PathBuf;

fn main() {
    env_logger::init();

    let args = Args::parse();
    debug!("Parsed Args: {:#?}", args);
    
    let filestructure = parse_fs(PathBuf::from(args.input_path), args.depth);
    debug!("Parsed filestructure: {:#?}", filestructure);
}
