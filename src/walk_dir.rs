use std::path;
// use walkdir::WalkDir;

use ignore::WalkBuilder;

use crate::file::File;
use crate::utils::get_file_size;

pub fn print_file_size(path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    for result in WalkBuilder::new(path)
        .hidden(!include_hidden)
        .git_ignore(!include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let file = File::new(String::from(path.to_str().unwrap()), get_file_size(path));
                    println!("{}", file);
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }
}
