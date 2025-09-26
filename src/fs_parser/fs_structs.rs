#[derive(Debug, Clone, Copy)]
pub enum FsEntryType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FlatFsEntry {
    pub name: String,
    pub entry_type: FsEntryType,
    pub depth: u32,
}
