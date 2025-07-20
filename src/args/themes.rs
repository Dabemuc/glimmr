use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Themes {
    Default,
    DefaultDark,
}

impl Themes {
    pub fn get_theme(&self) -> Theme {
        match self {
            Themes::Default => Theme {
                font: "Arial".into(),
                folder_font_size: 16,
                file_font_size: 14,
                folder_text_color: "hsl(0,0%,10%)".into(),
                file_text_color: "hsl(0,0%,15%)".into(),
                folder_bg_color: "hsl(0,0%,90%)".into(),
                file_bg_color: "hsl(0,0%,95%)".into(),
                bg_color: Some("hsl(0,0%,100%)".into()),
                bg_corner_rad: 0,
                folder_bg_corner_rad: 2,
                file_bg_corner_rad: 2,
            },
            Themes::DefaultDark => Theme {
                font: "Times New Roman".into(),
                folder_font_size: 14,
                file_font_size: 14,
                folder_text_color: "hsl(0,0%,90%)".into(),
                file_text_color: "hsl(0,0%,75%)".into(),
                folder_bg_color: "hsl(0,0%,10%)".into(),
                file_bg_color: "hsl(0,0%,15%)".into(),
                bg_color: Some("hsl(0,0%,5%)".into()),
                bg_corner_rad: 0,
                folder_bg_corner_rad: 2,
                file_bg_corner_rad: 2,
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
        write!(
            f,
            "{}",
            match self {
                Themes::Default => "default",
                Themes::DefaultDark => "default_dark",
            }
        )
    }
}

pub struct Theme {
    pub font: String,
    pub folder_font_size: u32,
    pub file_font_size: u32,
    pub folder_text_color: String,
    pub file_text_color: String,
    pub folder_bg_color: String,
    pub file_bg_color: String,
    pub bg_color: Option<String>,
    pub bg_corner_rad: u32,
    pub folder_bg_corner_rad: u32,
    pub file_bg_corner_rad: u32,
}
