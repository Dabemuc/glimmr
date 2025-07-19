use crate::fs_parser::fs_structs::{File, Folder, FsEntry};
use crate::args::themes::{Theme};
use rusttype::Font;
use log::debug;
use svg::{Document, node::Node};
use svg::node::element::{Group, Rectangle, Text};
mod fonts;
use fonts::{load_font_bytes, measure_text_width, build_b64_font_embed};

const LINE_HEIGHT: u32 = 20;
const LINE_PADDING: u32 = 5;
const DEPTH_OFFSET: u32 = 20;
const TOP_PADDING: u32 = 20;
const BG_X_PADDING: u32 = 20;
const ITEM_BG_PADDING: u32 = 3;

/// Trait that allows adding SVG nodes to any container (Document or Group)
pub trait AddableContainer {
    fn add_node(self, node: impl Node) -> Self;
}

impl AddableContainer for Document {
    fn add_node(mut self, node: impl Node) -> Self {
        self.append(node);
        self
    }
}

impl AddableContainer for Group {
    fn add_node(mut self, node: impl Node) -> Self {
        self.append(node);
        self
    }
}

/// Compose the full SVG from the folder structure
pub fn compose_svg_from_filestruct(
    filestructure: Folder,
    theme: Theme,
    include_root: bool,
    width: Option<u32>,
    heigth: Option<u32>
) -> Document {
    let mut doc = Document::new();

    // Load font and embed into svg
    let font_bytes = load_font_bytes(&theme.font)
        .expect(&format!("Failed to load system font '{}'", theme.font));
    let font = Font::try_from_vec(font_bytes.clone())
        .expect("Invalid font data");
    doc = doc.add(svg::node::element::Style::new(build_b64_font_embed(&font_bytes, &theme.font)));

    // Generate Background
    if let Some(bg) = &theme.bg_color {
        use svg::node::element::Rectangle;

        let background = Rectangle::new()
            .set("width", "100%")
            .set("height", "100%")
            .set("fill", bg.clone());

        doc = doc.add(background);
    }

    // Build filestructure visualization
    let mut leaf_count = 0;
    let mut max_depth = 0;
    let mut max_label_len_at_max_depth: f32 = 0.0;

    let root_node = if include_root {
        compose_folder_rec(filestructure, 0, 0, &mut leaf_count, &mut max_depth, &mut max_label_len_at_max_depth, &theme, &font)
            .set("transform", format!("translate({},{})", BG_X_PADDING, TOP_PADDING))
    } else {
        let contents = filestructure.contents;
        let group = Group::new();
        compose_rec(group, contents, 0, &mut leaf_count, &mut max_depth, &mut max_label_len_at_max_depth, &theme, &font)
            .set("transform", format!("translate({},{})", 0, 0))
    };

    debug!("Visualization has {} leafs and max depth {}", leaf_count, max_depth);
    doc = doc.add_node(root_node);

    // Define SVG size
    let computed_width = width.unwrap_or(
        max_label_len_at_max_depth as u32
        + ((max_depth-1) * DEPTH_OFFSET)
        + (BG_X_PADDING * 2)
    );
    let computed_height = heigth.unwrap_or(
        leaf_count * LINE_HEIGHT + TOP_PADDING
    );

    doc = doc.set("viewBox", (0, 0, computed_width, computed_height));

    doc
}

/// Recursively add folder contents into any AddableContainer
fn compose_rec<T>(
    mut container: T,
    contents: Vec<FsEntry>,
    depth: u32,
    leaf_count: &mut u32,
    max_depth: &mut u32,
    max_label_len_at_max_depth: &mut f32,
    theme: &Theme,
    font: &Font
) -> T
where
    T: AddableContainer,
{
    *max_depth = (*max_depth).max(depth);

    let mut curr_relative_y = 1;

    for child in contents {
        let before = *leaf_count;

        match child {
            FsEntry::Folder(sub_folder) => {
                if depth >= *max_depth {
                    *max_label_len_at_max_depth = (*max_label_len_at_max_depth).max(measure_text_width(&sub_folder.name, font, theme.folder_font_size as f32))
                }
                let group = compose_folder_rec(sub_folder, depth, curr_relative_y, leaf_count, max_depth, max_label_len_at_max_depth, theme, font);
                container = container.add_node(group);
            }
            FsEntry::File(file) => {
                if depth >= *max_depth {
                    *max_label_len_at_max_depth = (*max_label_len_at_max_depth).max(measure_text_width(&file.name, font, theme.folder_font_size as f32))
                }
                let text = compose_file(file, curr_relative_y, theme, font);
                container = container.add_node(text);
                *leaf_count += 1;
            }
        }

        curr_relative_y += *leaf_count - before;
    }

    container
}

/// Compose a folder and all its children into an SVG group
fn compose_folder_rec(
    folder: Folder,
    depth: u32,
    y_pos: u32,
    leaf_count: &mut u32,
    max_depth: &mut u32,
    max_label_len_at_max_depth: &mut f32,
    theme: &Theme,
    font: &Font
) -> Group {
    let Folder { name, contents } = folder;

    *max_depth = (*max_depth).max(depth);

    let bg = Rectangle::new()
        .set("y", -(theme.folder_font_size as i32))
        .set("fill", theme.folder_bg_color.clone())
        .set("width", measure_text_width(&name, font, theme.file_font_size as f32)*1.1 + ITEM_BG_PADDING as f32 * 2.0)
        .set("height", LINE_HEIGHT - LINE_PADDING + ITEM_BG_PADDING);

    let text = Text::new(name)
        .set("x", ITEM_BG_PADDING)
        .set("font-family", theme.font.clone())
        .set("font-size", theme.folder_font_size)
        .set("fill", theme.folder_text_color.clone());

    let group = Group::new()
        .set("transform", format!("translate({},{})", DEPTH_OFFSET, y_pos * LINE_HEIGHT))
        // .add(Text::new(format!("{} ({})", name, depth))
        .add(bg)
        .add(text);

    *leaf_count += 1;

    compose_rec(group, contents, depth + 1, leaf_count, max_depth, max_label_len_at_max_depth, theme, font)
}

/// Compose a single file into an SVG text element
fn compose_file(file: File, y_pos: u32, theme: &Theme, font: &Font) -> Group{
    let bg =  Rectangle::new()
        .set("y", -(theme.file_font_size as i32))
        .set("fill", theme.file_bg_color.clone())
        .set("width", measure_text_width(&file.name, font, theme.file_font_size as f32)*1.1 + ITEM_BG_PADDING as f32 * 2.0)
        .set("height", LINE_HEIGHT - LINE_PADDING + ITEM_BG_PADDING);

    let text = Text::new(file.name)
        .set("x", ITEM_BG_PADDING)
        .set("font-family", theme.font.clone())
        .set("font-size", theme.file_font_size)
        .set("fill", theme.file_text_color.clone());

    Group::new()
        .set("transform", format!("translate({},{})", DEPTH_OFFSET, y_pos * LINE_HEIGHT))
        .add(bg)
        .add(text)
}

