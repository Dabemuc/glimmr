pub mod fs_structs;
use fs_structs::{File, Folder, FsEntry};
use std::fs;
use std::io;
use std::path::PathBuf;

pub fn parse_fs(path: PathBuf, depth: u32) -> Folder {
    match crawl_dir_rec(path, depth as i32) {
        Ok(fs_tree) => {
            return fs_tree;
        }
        Err(e) => {
            panic!("Error while reading filestructure: {:?}", e);
        }
    }
}

fn crawl_dir_rec(path: PathBuf, depth: i32) -> io::Result<Folder> {
    let folder_name = path
        .file_name()
        .unwrap_or_else(|| path.as_os_str())
        .to_string_lossy()
        .into_owned();

    let mut contents: Vec<FsEntry> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path_to_entry = entry.path();

        if depth >= 0 {
            if path_to_entry.is_dir() {
                let sub_folder_node = crawl_dir_rec(path_to_entry, depth - 1)?;
                contents.push(FsEntry::Folder(sub_folder_node));
            } else {
                let file_name = path_to_entry
                    .file_name()
                    .unwrap_or_else(|| path_to_entry.as_os_str())
                    .to_string_lossy()
                    .into_owned();
                contents.push(FsEntry::File(File::new(file_name)));
            }
        }
    }

    Ok(Folder::new(folder_name, contents))
}
