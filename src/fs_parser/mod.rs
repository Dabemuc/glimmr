pub mod fs_structs;
use fs_structs::{FlatFsEntry, FsEntryType};
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn parse_fs_flat(path: PathBuf, max_depth: u32, include_root: bool) -> Vec<FlatFsEntry> {
    let mut flat_entries = Vec::new();

    if let Err(e) = crawl_dir_rec_flat(path, 0, max_depth as i32, include_root, &mut flat_entries) {
        panic!("Error while reading filestructure: {:?}", e);
    }

    flat_entries
}

fn crawl_dir_rec_flat(
    path: PathBuf,
    current_depth: u32,
    depth_left: i32,
    include_self: bool,
    flat_entries: &mut Vec<FlatFsEntry>,
) -> io::Result<()> {
    let folder_name = path
        .file_name()
        .unwrap_or_else(|| path.as_os_str())
        .to_string_lossy()
        .into_owned();

    if include_self {
        // Insert this folder itself
        flat_entries.push(FlatFsEntry {
            name: folder_name,
            entry_type: FsEntryType::Folder,
            depth: current_depth,
        });
    }

    if depth_left >= 0 {
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let path_to_entry = entry.path();

            if path_to_entry.is_dir() {
                // If root was not included, start children at current_depth (not +1)
                let next_depth = if include_self {
                    current_depth + 1
                } else {
                    current_depth
                };

                crawl_dir_rec_flat(
                    path_to_entry,
                    next_depth,
                    depth_left - 1,
                    true,
                    flat_entries,
                )?;
            } else {
                let file_name = path_to_entry
                    .file_name()
                    .unwrap_or_else(|| path_to_entry.as_os_str())
                    .to_string_lossy()
                    .into_owned();

                let file_depth = if include_self {
                    current_depth + 1
                } else {
                    current_depth
                };

                flat_entries.push(FlatFsEntry {
                    name: file_name,
                    entry_type: FsEntryType::File,
                    depth: file_depth,
                });
            }
        }
    }

    Ok(())
}
