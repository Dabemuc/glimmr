use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Filetype{
    SVG,
    PNG,
}

impl FromStr for Filetype{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "svg" => Ok(Filetype::SVG),
            "png" => Ok(Filetype::PNG),
            _ => Err("Invalid Filetype. Choose from 'svg', 'png'."),
        }
    }
}

impl std::fmt::Display for Filetype{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Filetype::SVG=> "svg",
            Filetype::PNG=> "png",
        })
    }
}
