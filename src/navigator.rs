use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

use crate::errors::AppError;

#[derive(Debug, Default)]
pub struct Navigator {
    pub root_dir: PathBuf,
    pub history: Vec<PathBuf>,
    pub dir_items: Vec<DirEntry>,
}

impl Navigator {
    pub fn new(path: PathBuf) -> Result<Navigator, AppError> {
        if path.exists() {
            let items = Navigator::get_dir_content(&path)?;
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

    pub fn change_root(&mut self, dir: PathBuf, add_to_history: bool) -> Result<(), AppError> {
        self.dir_items = Navigator::get_dir_content(&dir)?;
        if add_to_history {
            self.history.push(self.root_dir.clone());
        }
        self.root_dir = dir;
        Ok(())
    }

    // returns a vector of DirEntry, or an AppError on fail. Will not error out if a dir can't be read
    pub fn get_dir_content(path: &Path) -> Result<Vec<DirEntry>, AppError> {
        let mut dirs_vec: Vec<DirEntry> = Vec::new();
        let dirs = fs::read_dir(path).map_err(AppError::DirReadError)?;
        for dir in dirs {
            dirs_vec.push(dir.map_err(AppError::DirReadError)?);
        }
        Ok(dirs_vec)
    }
}
