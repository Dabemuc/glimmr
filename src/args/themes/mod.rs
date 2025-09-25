
use strum_macros::{Display, EnumIter, EnumString};

pub mod built_in;
use built_in::THEMES;

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Themes {
    BuiltIn(BuiltInThemes),
    Custom(Theme),
}

#[derive(Debug, Clone, Display, EnumIter, EnumString, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum BuiltInThemes {
    Default,
    DefaultDark,
    Tokyonight,
    Dracula,
    SolarizedDark,
    Monokai,
    Nord,
}

/// Theme configuration struct
#[derive(Debug, Clone, PartialEq)]
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
    pub hierarchy_line: HierarchyLineStyles,
    pub hierarchy_line_color: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HierarchyLineStyles {
    Default,
}

impl Themes {
    pub fn get_theme(&self) -> Theme {
        match self {
            Themes::BuiltIn(t) => THEMES.get(t).unwrap().clone(),
            Themes::Custom(custom) => custom.clone(),
        }
    }
}

impl BuiltInThemes {
    pub fn get_theme(&self) -> Theme {
        THEMES.get(self).unwrap().clone()
    }
}

pub mod custom;

impl std::str::FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, check if it matches a built-in theme
        if let Ok(builtin) = s.parse::<BuiltInThemes>() {
            return Ok(Themes::BuiltIn(builtin));
        }

        // If not built-in, try custom theme
        match custom::load_custom_theme(s) {
            Ok(theme) => Ok(Themes::Custom(theme)),
            Err(e) => Err(format!("Invalid theme input '{}'. Error: {}", s, e)),
        }
    }
}