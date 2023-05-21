use std::path;

use ignore::WalkBuilder;
use tabled::Table;

use crate::file::File;
use crate::utils::get_file_size;

pub fn print_file_size(path: path::PathBuf, include_hidden: bool, include_gitignored: bool) {
    let mut files = vec![];

    for result in WalkBuilder::new(path)
        .hidden(!include_hidden)
        .git_ignore(!include_gitignored)
        .build()
    {
        match result {
            Ok(entry) => {
                let path = entry.path();

                if path.is_file() {
                    let file_name = String::from(path.file_name().unwrap().to_str().unwrap());

                    let file = File::new(file_name, get_file_size(path));

                    files.push(file);
                }
            }

            Err(e) => println!("sz: error while reading path: {}", e),
        }
    }

    let table = Table::new(&files);
    println!("{}", table);
}
