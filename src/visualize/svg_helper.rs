use crate::fs_parser::fs_structs::{File, Folder, FsEntry};
use crate::args::themes::Theme;
use log::debug;
use svg::{Document, Node};
use svg::node::element::{Text, Group};

const LINE_HEIGHT: u32 = 20;
const DEPTH_OFFSET: u32 = 20;
const PADDING: u32 = 20;

pub fn compose_svg_from_filestruct(filestructure: Folder, _theme: Theme, include_root: bool) -> Document {
    let mut leaf_count = 0;
    let root_folder = compose_folder_rec(filestructure, 0, 2, &mut leaf_count);

    debug!("Visualization has {} leafs", leaf_count);
    let mut document = Document::new()
        .set("viewBox", (20, 20, 70, leaf_count * LINE_HEIGHT + PADDING));

    if include_root {
        document = document.add(root_folder);
    } else {
        // for child in root_folder.get_children() {
        //     document = document.add(child.cloned());
        // }
    }

    return document
}


fn compose_folder_rec(folder: Folder, depth: u32, y_pos: u32, leaf_count: &mut u32) -> Group{
    let mut group = Group::new()
        .set("transform", format!("translate({},{})", depth * DEPTH_OFFSET, y_pos * LINE_HEIGHT))
        .add(Text::new(folder.name)
            .set("font-family", "Arial")
            .set("font-size", 16)
            .set("fill", "blue"));

    *leaf_count += 1;

    let mut curr_relative_y = 1;

    for child in folder.contents {
        let relative_leaf_count_before_next_child = *leaf_count;
        match child {
            FsEntry::Folder(sub_folder) => {
                group = group.add(compose_folder_rec(sub_folder, depth+1, curr_relative_y, leaf_count));
            },
            FsEntry::File(sub_file) => {
                group = group.add(compose_file(sub_file, depth+1, curr_relative_y));
                *leaf_count += 1;
            },
        };
        curr_relative_y += *leaf_count - relative_leaf_count_before_next_child;
    };

    return group;
}


fn compose_file(file: File, depth: u32, y_pos: u32) -> Text {
    Text::new(file.name)
        .set("x", depth * DEPTH_OFFSET)
        .set("y", y_pos * LINE_HEIGHT)
        .set("font-family", "Arial")
        .set("font-size", 14)
        .set("fill", "green")
}
