use crate::fs_parser::fs_structs::{FlatFsEntry, FsEntryType};

fn collapse_folders(filestructure: Vec<FlatFsEntry>) -> Vec<FlatFsEntry> {
    let mut collapsed_structure = Vec::new();
    let mut i = 0;
    // Iterate through each entry in the filestructure
    while i < filestructure.len() {
        let current_entry = &filestructure[i];
        // Check if the current entry is a folder
        if let FsEntryType::Folder = current_entry.entry_type {
            // Find all direct children of the current folder
            let mut child_indices = Vec::new();
            let mut j = i + 1;
            while j < filestructure.len() && filestructure[j].depth > current_entry.depth {
                if filestructure[j].depth == current_entry.depth + 1 {
                    child_indices.push(j);
                }
                j += 1;
            }

            // If the folder has only one child and that child is also a folder, we start collapsing
            if child_indices.len() == 1 {
                let child_index = child_indices[0];
                if let FsEntryType::Folder = filestructure[child_index].entry_type {
                    let mut collapsed_name = current_entry.name.clone();
                    let mut last_collapsed_index = i;
                    let mut current_parent_index = i;

                    // Loop to build the chain of collapsible folders
                    loop {
                        // Find children of the current folder in the chain
                        let mut children_of_current_parent = Vec::new();
                        let mut k = current_parent_index + 1;
                        while k < filestructure.len() && filestructure[k].depth > filestructure[current_parent_index].depth {
                            if filestructure[k].depth == filestructure[current_parent_index].depth + 1 {
                                children_of_current_parent.push(k);
                            }
                            k += 1;
                        }

                        // If there is a single child and it's a folder, continue the chain
                        if children_of_current_parent.len() == 1 {
                            let single_child_index = children_of_current_parent[0];
                            if let FsEntryType::Folder = filestructure[single_child_index].entry_type {
                                collapsed_name.push_str(&format!("/{}", filestructure[single_child_index].name));
                                last_collapsed_index = single_child_index;
                                current_parent_index = single_child_index;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }

                    // Add the collapsed folder to the new structure
                    collapsed_structure.push(FlatFsEntry {
                        name: collapsed_name,
                        entry_type: FsEntryType::Folder,
                        depth: current_entry.depth,
                    });

                    // Adjust the depth of the children of the last collapsed folder
                    let depth_diff = filestructure[last_collapsed_index].depth - current_entry.depth;
                    let mut next_entry_index = last_collapsed_index + 1;
                    while next_entry_index < filestructure.len() && filestructure[next_entry_index].depth > filestructure[last_collapsed_index].depth {
                        let mut new_entry = filestructure[next_entry_index].clone();
                        new_entry.depth -= depth_diff;
                        collapsed_structure.push(new_entry);
                        next_entry_index += 1;
                    }

                    // Move the main iterator to the end of the processed entries
                    i = next_entry_index;
                    continue;
                }
            }
        }

        // If the entry is not a collapsible folder, just add it to the new structure
        collapsed_structure.push(filestructure[i].clone());
        i += 1;
    }

    collapsed_structure
}

pub fn do_transforms(
    filestructure: Vec<FlatFsEntry>,
    collapse_folders_flag: bool,
) -> Vec<FlatFsEntry> {
    if collapse_folders_flag {
        return collapse_folders(filestructure);
    }
    return filestructure;
}
