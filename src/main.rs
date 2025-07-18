mod args;
use args::Args;
use clap::Parser;
use log::{debug};

fn main() {
    let args = Args::parse();

    debug!("Parsed Args: {:?}", args);
}
