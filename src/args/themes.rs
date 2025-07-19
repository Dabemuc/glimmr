use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Themes {
    Default,
    DefaultDark
}

impl Themes {
    pub fn get_theme(&self) -> Theme {
        match self {
            Themes::Default =>  Theme {
                font_family: "Arial".into(),
                folder_font_size: 16,
                file_font_size: 14,
                folder_color: "blue".into(),
                file_color: "green".into(),
                background_color: None,
            },
            Themes::DefaultDark => Theme {
                font_family: "Fira Code".into(),
                folder_font_size: 16,
                file_font_size: 14,
                folder_color: "lightblue".into(),
                file_color: "lightgreen".into(),
                background_color: Some("black".into()),
            },
        }
    }
}

impl FromStr for Themes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "default" => Ok(Themes::Default),
            "default_dark" => Ok(Themes::DefaultDark),
            _ => Err("Invalid theme. Choose from 'default', 'default_dark'."),
        }
    }
}

impl std::fmt::Display for Themes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Themes::Default => "default",
            Themes::DefaultDark=> "default_dark",
        })
    }
}

pub struct Theme {
    pub font_family: String,
    pub folder_font_size: u32,
    pub file_font_size: u32,
    pub folder_color: String,
    pub file_color: String,
    pub background_color: Option<String>,
}
