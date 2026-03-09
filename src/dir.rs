use std::{fs::DirEntry, path::PathBuf};

use crate::{errors::AppError, utils};

#[derive(Debug, Default)]
pub struct Dir {
    pub root_dir: PathBuf,
    pub history: Vec<PathBuf>,
    pub dir_items: Vec<DirEntry>,
    pub items_changed: bool,
}

impl Dir {
    pub fn new(path: PathBuf) -> Result<Dir, AppError> {
        if path.exists() {
            let items = utils::get_dir_content(&path).unwrap_or_default();
            let dir = Dir {
                root_dir: path,
                dir_items: items,
                ..Default::default()
            };
            Ok(dir)
        } else {
            Err(AppError::GhostPath)
        }
    }
}
