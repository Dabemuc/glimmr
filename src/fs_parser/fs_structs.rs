#[derive(Debug)]
pub enum FsEntryType {
    File,
    Folder,
}

#[derive(Debug)]
pub struct FlatFsEntry {
    pub name: String,
    pub entry_type: FsEntryType,
    pub depth: u32,
}
