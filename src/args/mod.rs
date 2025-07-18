mod themes;
use themes::Theme;
mod filetypes;
use filetypes::Filetype;
use clap::Parser;

/// # Glimmr
/// Create beautiful visualisations of filestructures, fast and easy.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Theme to use
    #[arg(short, long, default_value_t = Theme::Default)]
    theme: Theme,

    /// Output filetype
    #[arg(short, long, default_value_t = Filetype::SVG)]
    filetype: Filetype,

    /// Name of output file
    #[arg(short, long, default_value = "glimmr_out")]
    output_filename: String,
}
