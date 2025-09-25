use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::{BuiltInThemes, HierarchyLineStyles, Theme};

/// Static map containing all theme configurations
pub static THEMES: Lazy<HashMap<BuiltInThemes, Theme>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        BuiltInThemes::Default,
        Theme {
            font: "Arial".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "#111111".into(),
            file_text_color: "#333333".into(),
            folder_bg_color: "#F5F5F5".into(),
            file_bg_color: "#FAFAFA".into(),
            bg_color: Some("#FFFFFF".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "#555555".into(),
        },
    );

    m.insert(
        BuiltInThemes::DefaultDark,
        Theme {
            font: "Arial".into(),
            folder_font_size: 14,
            file_font_size: 14,
            folder_text_color: "#FFFFFF".into(),
            file_text_color: "#CCCCCC".into(),
            folder_bg_color: "#333333".into(),
            file_bg_color: "#222222".into(),
            bg_color: Some("#111111".into()),
            bg_corner_rad: 0,
            folder_bg_corner_rad: 2,
            file_bg_corner_rad: 2,
            hierarchy_line: HierarchyLineStyles::Default,
            hierarchy_line_color: "#DDDDDD".into(),
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

