use super::{BuiltInThemes, Theme};
use serde::Deserialize;
use std::fs;
use std::path::Path;

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
