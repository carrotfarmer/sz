use std::path::Path;

use ignore::WalkBuilder;

use crate::Args;

pub fn get_file_size(path: &Path) -> f64 {
    match path.metadata() {
        Ok(metadata) => metadata.len() as f64,
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0.0
        }
    }
}

pub fn get_dir_size(path: &Path, args: Args) -> f64 {
    let mut dir_size: f64 = 0.0;

    for item in WalkBuilder::new(path)
        .hidden(!args.include_hidden)
        .git_exclude(!args.include_gitignored)
        .build()
    {
        match item {
            Ok(item) => {
                if item.path().is_file() {
                    dir_size += get_file_size(item.path());
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    dir_size
}
