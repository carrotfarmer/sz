use std::path;
use walkdir::WalkDir;

use crate::file::File;
use crate::utils::get_file_size;

pub fn print_file_size(path: path::PathBuf) {
    // i'm sorry
    let path_path = path.as_path();

    for thingy in WalkDir::new(path.clone()) {
        let thingy = thingy.unwrap();

        if thingy.path().is_file() {
            let file_size = get_file_size(thingy.path());

            let file = File::new(String::from(thingy.path().to_str().unwrap()), file_size);
            file.print();
        }
    }
}
