pub mod themes;
use themes::Themes;
pub mod filetypes;
use filetypes::Filetype;
use clap::Parser;
use std::path::PathBuf;

/// # Glimmr
/// Create beautiful visualisations of filestructures, fast and easy.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the directory to visualize
    #[arg()]
    pub input_path: PathBuf,

    /// Theme to use
    #[arg(short, long, default_value_t = Themes::Default)]
    pub theme: Themes,

    /// Output filetype
    #[arg(short, long, default_value_t = Filetype::SVG)]
    pub filetype: Filetype,

    /// Output filename or filepath
    #[arg(short, long, default_value = "glimmr_out")]
    pub output_filepath: String,

    /// Max recursive depth
    #[arg(short, long, default_value_t = 3)]
    pub depth: u32,

    /// Include root folder
    #[arg(short = 'r', long)]
    pub include_root: bool,

    /// Dont render background color
    #[arg(long)]
    pub bg_transparent: bool,
}
