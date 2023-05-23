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

pub fn get_dir_size(path: &Path, args: Args) -> (f64, usize) {
    let mut dir_size: f64 = 0.0;
    let mut files_count = 0;

    for item in WalkBuilder::new(path)
        .hidden(!args.include_hidden)
        .git_exclude(!args.include_gitignored)
        .build()
    {
        match item {
            Ok(item) => {
                if item.path().is_file() {
                    dir_size += get_file_size(item.path());
                    files_count += 1;
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    (dir_size, files_count)
}

pub fn shorten_name(item_name: String) -> String {
    if item_name.len() > 35 {
        let mut s = item_name 
            .chars()
            .rev()
            .take(30)
            .collect::<String>()
            .chars()
            .rev()
            .collect::<String>();

        s.insert_str(0, "...");

        return s;
    }

    item_name
}
