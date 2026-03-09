use std::{fs::DirEntry, path::PathBuf};

pub struct Dir {
    pub root_dir: PathBuf,
    pub history: Vec<PathBuf>,
    pub dir_items: Vec<DirEntry>,
    pub items_changed: bool,
}
