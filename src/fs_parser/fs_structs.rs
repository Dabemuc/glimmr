use std::path::PathBuf;

#[derive(Debug)]
pub struct File {
    name: String,
}
impl File {
    pub fn new(name: String) -> Self {
        File { name }
    }
}

#[derive(Debug)]
pub struct Folder {
    name: String,
    contents: Vec<FsEntry>,
}
impl Folder {
    pub fn new(name: String, contents: Vec<FsEntry>) -> Self {
        Folder { name, contents }
    }
}

#[derive(Debug)]
pub enum FsEntry {
    File(File),
    Folder(Folder),
}
