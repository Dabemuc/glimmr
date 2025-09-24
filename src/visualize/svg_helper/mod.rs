use crate::args::themes::{HierarchyLineStyles, Theme};
use crate::fs_parser::fs_structs::{FlatFsEntry, FsEntryType};
use crate::visualize::svg_helper::fonts::{
    build_b64_font_embed, load_font_bytes, measure_text_width,
};
use rusttype::Font;
use svg::Document;
use svg::node::element::path::Data;
use svg::node::element::{Group, Path, Rectangle, Script, Text};
mod fonts;
use svg::Node;

const ROW_HEIGHT: u32 = 20;
const ROW_PADDING: u32 = 3;
const DEPTH_OFFSET: u32 = 20;
const TOP_PADDING: u32 = 20;
const BG_X_PADDING: u32 = 20;
const ITEM_BG_X_PADDING: u32 = 3;
const ITEM_BG_Y_PADDING: u32 = 1;

/// Compose the full SVG from the folder structure
pub fn compose_svg_from_filestruct(
    filestructure: Vec<FlatFsEntry>,
    theme: Theme,
    width: Option<u32>,
    heigth: Option<u32>,
    bake_font: bool,
) -> Document {
    let mut doc = Document::new();

    // Load font and embed into svg
    let font_bytes = load_font_bytes(&theme.font)
        .expect(&format!("Failed to load system font '{}'", theme.font));
    let font = Font::try_from_vec(font_bytes.clone()).expect("Invalid font data");
    if bake_font {
        doc = doc.add(svg::node::element::Style::new(build_b64_font_embed(
            &font_bytes,
            &theme.font,
        )));
    }

    // Generate Background
    if let Some(bg) = &theme.bg_color {
        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", bg.clone())
            .set("rx", theme.bg_corner_rad)
            .set("ry", theme.bg_corner_rad);

        doc = doc.add(background);
    }

    // Build hierarchy lines
    for (i, entry) in filestructure.iter().enumerate() {
        // Build hierarchy lines for this row
        for d in 0..entry.depth {
            let mut is_last = false;
            if i == filestructure.len() - 1 {
                is_last = true;
            } else if filestructure[i + 1].depth < d + 1 {
                is_last = true
            }

            doc = doc.add(compose_hierarchy_line(
                DEPTH_OFFSET / 2 + DEPTH_OFFSET * d + BG_X_PADDING,
                (ROW_HEIGHT + ROW_PADDING) * (i - 1) as u32 + TOP_PADDING,
                is_last,
                &theme,
            ))
        }
    }

    // Build filestructure visualization
    let mut max_depth = 0;
    let mut max_label_len_at_max_depth = 0;
    for (i, entry) in filestructure.iter().enumerate() {
        // Build file/folder for this row
        let row_x = DEPTH_OFFSET * entry.depth + BG_X_PADDING;
        let row_y = (ROW_HEIGHT + ROW_PADDING) * i as u32 + TOP_PADDING;
        match entry.entry_type {
            FsEntryType::File => {
                doc = doc.add(compose_file(&entry.name, row_x, row_y, &theme, &font))
            }
            FsEntryType::Folder => {
                doc = doc.add(compose_folder(&entry.name, row_x, row_y, &theme, &font))
            }
        }

        // Update max vars
        if entry.depth > max_depth {
            max_depth = entry.depth
        }
        let computed_text_len = measure_text_width(&entry.name, &font, theme.file_font_size as f32);
        if computed_text_len as i32 > max_label_len_at_max_depth {
            max_label_len_at_max_depth = computed_text_len as i32
        }
    }

    // Add script to get widths correct
    let script_content = format!(
        r#"
    function adjustBoxes() {{
        document.querySelectorAll('g.file, g.folder').forEach(group => {{
            const text = group.querySelector('text.label-text');
            const rect = group.querySelector('rect.label-bg');
            if (text && rect) {{
                const bbox = text.getBBox();
                const x_padding = {};
                const y_padding = {};
                rect.setAttribute('width', bbox.width + x_padding);
                rect.setAttribute('height', bbox.height + y_padding);
            }}
        }});
    }}
    adjustBoxes();
    "#,
        ITEM_BG_X_PADDING * 2,
        ITEM_BG_Y_PADDING * 2
    );

    let mut script = Script::new(script_content);
    script.assign("type", "application/ecmascript");

    doc = doc.add(script);

    // Define SVG size
    let computed_width = width.unwrap_or(
        max_label_len_at_max_depth as u32 + (max_depth * DEPTH_OFFSET) + (BG_X_PADDING * 2),
    );
    let computed_height =
        heigth.unwrap_or(filestructure.len() as u32 * (ROW_HEIGHT + ROW_PADDING) + TOP_PADDING);
    doc = doc.set("viewBox", (0, 0, computed_width, computed_height));

    doc
}

/// Compose a file SVG element
fn compose_file(name: &str, x_pos: u32, y_pos: u32, theme: &Theme, _font: &Font) -> Group {
    let bg = Rectangle::new()
        .set("class", "label-bg")
        .set("y", -(theme.file_font_size as i32))
        .set("height", ROW_HEIGHT)
        .set("rx", theme.file_bg_corner_rad)
        .set("ry", theme.file_bg_corner_rad)
        .set("fill", theme.file_bg_color.clone());

    let text = Text::new(name)
        .set("class", "label-text")
        .set("x", ITEM_BG_X_PADDING)
        .set("font-family", theme.font.clone())
        .set("font-size", theme.file_font_size)
        .set("fill", theme.file_text_color.clone());

    Group::new()
        .set("class", "file")
        .set("transform", format!("translate({},{})", x_pos, y_pos))
        .add(bg)
        .add(text)
}

/// Compose a folder SVG element
fn compose_folder(name: &str, x_pos: u32, y_pos: u32, theme: &Theme, _font: &Font) -> Group {
    let bg = Rectangle::new()
        .set("class", "label-bg")
        .set("y", -(theme.folder_font_size as i32))
        .set("height", ROW_HEIGHT)
        .set("rx", theme.folder_bg_corner_rad)
        .set("ry", theme.folder_bg_corner_rad)
        .set("fill", theme.folder_bg_color.clone());

    let text = Text::new(name)
        .set("class", "label-text")
        .set("x", ITEM_BG_X_PADDING)
        .set("font-family", theme.font.clone())
        .set("font-size", theme.folder_font_size)
        .set("fill", theme.folder_text_color.clone());

    Group::new()
        .set("class", "folder")
        .set("transform", format!("translate({},{})", x_pos, y_pos))
        .add(bg)
        .add(text)
}

pub fn compose_hierarchy_line(x_pos: u32, y_pos: u32, is_last: bool, theme: &Theme) -> Path {
    match theme.hierarchy_line {
        HierarchyLineStyles::Default => {
            let mut data = Data::new().move_to((x_pos, y_pos));

            // Display horizontal line when is last
            if is_last {
                data = data
                    .vertical_line_by(ROW_HEIGHT + ROW_PADDING)
                    .horizontal_line_by(DEPTH_OFFSET as f32 * 0.8);
            } else {
                data = data.vertical_line_by(ROW_HEIGHT + ROW_PADDING);
            }

            let path = Path::new()
                .set("fill", "none")
                .set("stroke", theme.hierarchy_line_color.clone())
                .set("stroke-width", 1)
                .set("d", data);

            return path;
        }
    }
}
