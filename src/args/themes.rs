use once_cell::sync::Lazy;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/// Enum representing available themes.
/// Adding a new theme requires only adding it here and defining its Theme config.
#[derive(Debug, Clone, Display, EnumIter, PartialEq, Eq, Hash)]
#[strum(serialize_all = "snake_case")]
pub enum Themes {
    Default,
    DefaultDark,
    Tokyonight,
    Dracula,
    SolarizedDark,
    Monokai,
    Nord,
}

/// Theme configuration struct
#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum HierarchyLineStyles {
    Default,
}

/// Static map containing all theme configurations
static THEMES: Lazy<HashMap<Themes, Theme>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        Themes::Default,
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
        Themes::DefaultDark,
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
        Themes::Tokyonight,
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
        Themes::Dracula,
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
        Themes::SolarizedDark,
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
        Themes::Monokai,
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
        Themes::Nord,
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
    /// Returns the Theme configuration for this enum
    pub fn get_theme(&self) -> Theme {
        THEMES.get(self).unwrap().clone()
    }
}

/// Implement FromStr dynamically using strum iteration
impl std::str::FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Themes::iter()
            .find(|t| t.to_string() == s.to_lowercase())
            .ok_or_else(|| {
                let all = Themes::iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("Invalid theme. Choose from: {}", all)
            })
    }
}
