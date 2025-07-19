use crate::fs_parser::fs_structs::{File, Folder, FsEntry};
use crate::args::themes::{Theme};
use log::debug;
use svg::{Document, node::Node};
use svg::node::element::{Text, Group};

const LINE_HEIGHT: u32 = 20;
const DEPTH_OFFSET: u32 = 20;
const TOP_PADDING: u32 = 20;
const CHAR_WIDTH_ESTIMATE: u32 = 8;
const X_PADDING: u32 = 20;

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
pub fn compose_svg_from_filestruct(filestructure: Folder, theme: Theme, include_root: bool, width: Option<u32>, heigth: Option<u32>) -> Document {
    let mut doc = Document::new();

    // Generate Background
    if let Some(bg) = &theme.background_color {
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
    let mut max_label_len_at_max_depth: usize = 0;

    let root_node = if include_root {
        compose_folder_rec(filestructure, 0, 0, &mut leaf_count, &mut max_depth, &mut max_label_len_at_max_depth, &theme)
            .set("transform", format!("translate({},{})", X_PADDING, TOP_PADDING))
    } else {
        let contents = filestructure.contents;
        let group = Group::new();
        compose_rec(group, contents, 0, &mut leaf_count, &mut max_depth, &mut max_label_len_at_max_depth, &theme)
            .set("transform", format!("translate({},{})", X_PADDING, 0))
    };

    debug!("Visualization has {} leafs and max depth {}", leaf_count, max_depth);
    doc = doc.add_node(root_node);

    // Define SVG size
    let computed_width = width.unwrap_or(
        (max_depth * DEPTH_OFFSET)
        + (max_label_len_at_max_depth as u32 * CHAR_WIDTH_ESTIMATE)
        + (X_PADDING * 2)
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
    max_label_len_at_max_depth: &mut usize,
    theme: &Theme
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
                    *max_label_len_at_max_depth = (*max_label_len_at_max_depth).max(sub_folder.name.len());
                }
                let group = compose_folder_rec(sub_folder, depth, curr_relative_y, leaf_count, max_depth, max_label_len_at_max_depth, theme);
                container = container.add_node(group);
            }
            FsEntry::File(file) => {
                if depth >= *max_depth {
                    *max_label_len_at_max_depth = (*max_label_len_at_max_depth).max(file.name.len());
                }
                let text = compose_file(file, curr_relative_y, theme);
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
    max_label_len_at_max_depth: &mut usize,
    theme: &Theme
) -> Group {
    let Folder { name, contents } = folder;

    *max_depth = (*max_depth).max(depth);

    let group = Group::new()
        .set("transform", format!("translate({},{})", DEPTH_OFFSET, y_pos * LINE_HEIGHT))
        // .add(Text::new(format!("{} ({})", name, depth))
        .add(Text::new(name)
            .set("font-family", theme.font_family.clone())
            .set("font-size", theme.folder_font_size)
            .set("fill", theme.folder_color.clone())
        );

    *leaf_count += 1;

    compose_rec(group, contents, depth + 1, leaf_count, max_depth, max_label_len_at_max_depth, theme)
}

/// Compose a single file into an SVG text element
fn compose_file(file: File, y_pos: u32, theme: &Theme) -> Text {
    Text::new(file.name)
        .set("x", DEPTH_OFFSET)
        .set("y", y_pos * LINE_HEIGHT)
        .set("font-family", theme.font_family.clone())
        .set("font-size", theme.file_font_size)
        .set("fill", theme.file_color.clone())
}
