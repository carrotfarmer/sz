use std::fs;
use std::path::{Component, Path};

use walkdir::{DirEntry, WalkDir};

pub fn get_file_size(path: &Path) -> u64 {
    match path.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0
        }
    }
}

// check if a path/file is hidden
pub fn is_hidden(path: &Path) -> bool {
    path.file_stem().unwrap().to_string_lossy().starts_with('.') || {
        let parent_dir = path.parent();
        if let Some(parent) = parent_dir {
            for component in parent.components() {
                if let Component::Normal(name) = component {
                    if name.to_string_lossy().starts_with('.') {
                        return true;
                    }
                }
            }
        }

        false
    }
}
