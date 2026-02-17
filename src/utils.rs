use std::{
    fs::{DirEntry, read_dir},
    path::Path,
};

use crate::errors::AppError;

// returns a vector of DirEntry, or an AppError on fail. Will not error out if a dir can't be read
pub fn get_dir_content(path: &Path) -> Result<Vec<DirEntry>, AppError> {
    let mut dirs_vec: Vec<DirEntry> = Vec::new();
    let dirs = read_dir(path).map_err(|e| AppError::DirReadError(e))?;
    for dir in dirs {
        if let Ok(dir) = dir {
            dirs_vec.push(dir);
        } else {
            todo!("add logic for dirs that couldn't be read");
        }
    }
    Ok(dirs_vec)
}

pub fn format_size(dir: &DirEntry) -> String {
    if let Ok(metadata) = dir.metadata() {
        let mut size = metadata.len();
        let mut size_fmt = String::from("----");
        for unit in ["B", "KB", "MB", "GB", "TB"] {
            if size < 1024 {
                size_fmt = size.to_string() + unit;
                break;
            }
            size /= 1024;
        }
        return size_fmt;
    } else {
        String::from("----")
    }
}
