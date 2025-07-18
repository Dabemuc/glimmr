use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Theme {
    Default,
    TokioNight
}

impl FromStr for Theme {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Theme::Default),
            "tokio_night" => Ok(Theme::TokioNight),
            _ => Err("Invalid theme. Choose from 'default', 'tokio_night'."),
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Theme::Default => "default",
            Theme::TokioNight=> "tokio_night",
        })
    }
}
