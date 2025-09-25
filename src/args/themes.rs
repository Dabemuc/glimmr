use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use strum_macros::{Display, EnumIter, EnumString};

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

/// Static map containing all theme configurations
static THEMES: Lazy<HashMap<BuiltInThemes, Theme>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        BuiltInThemes::Default,
        Theme {
            font: "Arial".into(),
            folder_font_size: 13,
            file_font_size: 12,
            folder_text_color: "hsl(0,0%,10%)".into(),
            file_text_color: "hsl(0,0%,15%)".into(),
            folder_bg_color: "hsl(0,0%,90%)".into(),
            file_bg_color: "hsl(0,0%,95%)".into(),
            bg_color: Some("hsl(0,0%,100%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(0,0%,25%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::DefaultDark,
        Theme {
            font: "Arial".into(),
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
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(0,0%,55%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::Tokyonight,
        Theme {
            font: "Fira Code".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "hsl(203, 74%, 82%)".into(),
            file_text_color: "hsl(203, 74%, 70%)".into(),
            folder_bg_color: "hsl(203, 50%, 18%)".into(),
            file_bg_color: "hsl(203, 50%, 12%)".into(),
            bg_color: Some("hsl(203, 50%, 8%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 3,
            file_bg_corner_rad: 3,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(203, 50%, 45%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::Dracula,
        Theme {
            font: "Courier New".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "hsl(288, 50%, 85%)".into(),
            file_text_color: "hsl(288, 50%, 70%)".into(),
            folder_bg_color: "hsl(282, 50%, 15%)".into(),
            file_bg_color: "hsl(282, 50%, 10%)".into(),
            bg_color: Some("hsl(282, 50%, 5%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(288, 50%, 60%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::SolarizedDark,
        Theme {
            font: "Menlo".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "hsl(45, 60%, 80%)".into(),
            file_text_color: "hsl(45, 60%, 70%)".into(),
            folder_bg_color: "hsl(200, 30%, 15%)".into(),
            file_bg_color: "hsl(200, 30%, 10%)".into(),
            bg_color: Some("hsl(200, 30%, 5%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(45, 60%, 55%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::Monokai,
        Theme {
            font: "Consolas".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "hsl(48, 100%, 85%)".into(),
            file_text_color: "hsl(48, 100%, 70%)".into(),
            folder_bg_color: "hsl(290, 60%, 15%)".into(),
            file_bg_color: "hsl(290, 60%, 10%)".into(),
            bg_color: Some("hsl(290, 60%, 5%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(48, 100%, 60%)".into(),
        },
    );

    m.insert(
        BuiltInThemes::Nord,
        Theme {
            font: "Ubuntu Mono".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "hsl(210, 30%, 90%)".into(),
            file_text_color: "hsl(210, 30%, 75%)".into(),
            folder_bg_color: "hsl(210, 25%, 20%)".into(),
            file_bg_color: "hsl(210, 25%, 15%)".into(),
            bg_color: Some("hsl(210, 25%, 10%)".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "hsl(210, 30%, 55%)".into(),
        },
    );

    m
});

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

impl std::str::FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, check if it matches a built-in theme
        if let Ok(builtin) = s.parse::<BuiltInThemes>() {
            return Ok(Themes::BuiltIn(builtin));
        }

        // If not built-in, try custom theme
        match load_custom_theme(s) {
            Ok(theme) => Ok(Themes::Custom(theme)),
            Err(e) => Err(format!("Invalid theme input '{}'. Error: {}", s, e)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct PartialTheme {
    pub base_theme: Option<String>,
    pub font: Option<String>,
    pub folder_font_size: Option<u32>,
    pub file_font_size: Option<u32>,
    pub folder_text_color: Option<String>,
    pub file_text_color: Option<String>,
    pub folder_bg_color: Option<String>,
    pub file_bg_color: Option<String>,
    pub bg_color: Option<String>,
    pub bg_corner_rad: Option<u32>,
    pub folder_bg_corner_rad: Option<u32>,
    pub file_bg_corner_rad: Option<u32>,
    pub hierarchy_line_color: Option<String>,
}

impl Theme {
    pub fn apply_overlay(&self, overlay: &PartialTheme) -> Theme {
        Theme {
            font: overlay.font.clone().unwrap_or_else(|| self.font.clone()),
            folder_font_size: overlay.folder_font_size.unwrap_or(self.folder_font_size),
            file_font_size: overlay.file_font_size.unwrap_or(self.file_font_size),
            folder_text_color: overlay
                .folder_text_color
                .clone()
                .unwrap_or_else(|| self.folder_text_color.clone()),
            file_text_color: overlay
                .file_text_color
                .clone()
                .unwrap_or_else(|| self.file_text_color.clone()),
            folder_bg_color: overlay
                .folder_bg_color
                .clone()
                .unwrap_or_else(|| self.folder_bg_color.clone()),
            file_bg_color: overlay
                .file_bg_color
                .clone()
                .unwrap_or_else(|| self.file_bg_color.clone()),
            bg_color: overlay.bg_color.clone().or_else(|| self.bg_color.clone()),
            bg_corner_rad: overlay.bg_corner_rad.unwrap_or(self.bg_corner_rad),
            folder_bg_corner_rad: overlay
                .folder_bg_corner_rad
                .unwrap_or(self.folder_bg_corner_rad),
            file_bg_corner_rad: overlay
                .file_bg_corner_rad
                .unwrap_or(self.file_bg_corner_rad),
            hierarchy_line: self.hierarchy_line.clone(),
            hierarchy_line_color: overlay
                .hierarchy_line_color
                .clone()
                .unwrap_or_else(|| self.hierarchy_line_color.clone()),
        }
    }
}

/// Load a custom theme from JSON string or file path
pub fn load_custom_theme(input: &str) -> Result<Theme, String> {
    let json_str = if Path::new(input).exists() {
        fs::read_to_string(input).map_err(|e| format!("Failed to read file: {}", e))?
    } else {
        input.to_string()
    };

    let partial: PartialTheme =
        serde_json::from_str(&json_str).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let base_theme = if let Some(base_name) = &partial.base_theme {
        match base_name.parse::<BuiltInThemes>() {
            Ok(theme_enum) => theme_enum.get_theme(),
            Err(_) => return Err(format!("Invalid base_theme: {}", base_name)),
        }
    } else {
        BuiltInThemes::Default.get_theme()
    };

    Ok(base_theme.apply_overlay(&partial))
}
