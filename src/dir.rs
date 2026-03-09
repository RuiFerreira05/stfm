use std::{fs::DirEntry, path::PathBuf};

use crate::{errors::AppError, utils};

#[derive(Debug, Default)]
pub struct Navigator {
    pub root_dir: PathBuf,
    pub history: Vec<PathBuf>,
    pub dir_items: Vec<DirEntry>,
    pub items_changed: bool,
}

impl Navigator {
    pub fn new(path: PathBuf) -> Result<Navigator, AppError> {
        if path.exists() {
            let items = utils::get_dir_content(&path).unwrap_or_default();
            let dir = Navigator {
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
