use std::path;
// use walkdir::WalkDir;

use ignore::Walk;

use crate::file::File;
use crate::utils::{get_file_size, is_hidden};

pub fn print_file_size(path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    for result in Walk::new(path) {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let file = File::new(String::from(path.to_str().unwrap()), get_file_size(path))
                        .print();
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }
}
