pub mod themes;
use themes::Theme;
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
    #[arg(short, long, default_value_t = Theme::Default)]
    pub theme: Theme,

    /// Output filetype
    #[arg(short, long, default_value_t = Filetype::SVG)]
    pub filetype: Filetype,

    /// Output filename or filepath
    #[arg(short, long, default_value = ".")]
    pub output_filepath: String,

    /// Max recursive depth
    #[arg(short, long, default_value_t = 3)]
    pub depth: u32,

    /// Include root folder
    #[arg(short = 'r', long)]
    pub include_root: bool,
}
