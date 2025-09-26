pub mod fs_structs;
use fs_structs::{FlatFsEntry, FsEntryType};
use ignore::{WalkBuilder, overrides::OverrideBuilder};
use std::path::PathBuf;

/// Parses the filesystem starting from `path` and returns a flat vector of `FlatFsEntry`.
///
/// This function uses the `ignore` crate to walk the directory tree. By default, it does
/// not respect `.gitignore` files or hidden files (dotfiles). This can be enabled with
/// the `use_gitignore` and `ignore_hidden` flags. It also allows for custom exclusion patterns.
///
/// # Arguments
///
/// * `path` - The starting path for the directory traversal.
/// * `max_depth` - The maximum depth to traverse, relative to the root.
/// * `include_root` - Whether to include the starting path itself in the output.
/// * `excludes` - A vector of `PathBuf`s to exclude from the traversal. These are treated as glob patterns.
/// * `use_gitignore` - Whether to respect `.gitignore` files for exclusion.
/// * `ignore_hidden` - Whether to ignore hidden files and directories.
pub fn parse_fs_flat(
    path: PathBuf,
    max_depth: u32,
    include_root: bool,
    excludes: Vec<PathBuf>,
    use_gitignore: bool,
    ignore_hidden: bool,
) -> Vec<FlatFsEntry> {
    let mut flat_entries = Vec::new();

    // Create a WalkBuilder to configure the directory traversal.
    let mut walk_builder = WalkBuilder::new(&path);

    // Configure ignore settings. By default, we don't use .gitignore or ignore hidden files.
    if !use_gitignore {
        walk_builder.git_ignore(false);
    }
    walk_builder.hidden(ignore_hidden);

    // Create an OverrideBuilder to add custom exclusion patterns.
    let mut override_builder = OverrideBuilder::new(&path);
    for exclude in excludes {
        if let Err(e) = override_builder.add(&exclude.to_string_lossy()) {
            eprintln!("Could not add exclude pattern: {}", e);
        }
    }
    if let Ok(overrides) = override_builder.build() {
        walk_builder.overrides(overrides);
    }

    // Adjust the max depth for the WalkBuilder based on whether the root is included.
    // If the root is not included, we need to go one level deeper to get the same number of levels.
    let final_max_depth = if include_root {
        max_depth as usize
    } else {
        (max_depth + 1) as usize
    };
    walk_builder.max_depth(Some(final_max_depth));

    // Build and iterate over the directory walker.
    for result in walk_builder.build() {
        match result {
            Ok(entry) => {
                let mut depth = entry.depth() as u32;

                // If the root is not included, skip the root entry (depth 0) and decrement
                // the depth of all other entries.
                if !include_root {
                    if entry.depth() == 0 {
                        continue;
                    }
                    depth -= 1;
                }

                // Skip entries that are deeper than the specified max_depth.
                if depth > max_depth {
                    continue;
                }

                // Determine the entry type (File or Folder).
                let entry_type = if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    FsEntryType::Folder
                } else {
                    FsEntryType::File
                };

                let name = entry.file_name().to_string_lossy().into_owned();

                // Add the entry to our flat list.
                flat_entries.push(FlatFsEntry {
                    name,
                    entry_type,
                    depth,
                });
            }
            Err(err) => eprintln!("ERROR: {}", err),
        }
    }

    flat_entries
}
