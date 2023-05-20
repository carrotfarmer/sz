use std::path;
use walkdir::WalkDir;

use crate::file::File;
use crate::utils::{get_file_size, is_hidden};

pub fn print_file_size(path: path::PathBuf, include_hidden: bool) {
    for thingy in WalkDir::new(path.clone()) {
        let thingy = thingy.unwrap();

        if thingy.path().is_file() {
            match include_hidden {
                false => {
                    if !is_hidden(thingy.path()) {
                        let file_size = get_file_size(thingy.path());

                        let file =
                            File::new(String::from(thingy.path().to_str().unwrap()), file_size);
                        file.print();
                    }
                }

                true => {
                    let file_size = get_file_size(thingy.path());

                    let file = File::new(String::from(thingy.path().to_str().unwrap()), file_size);
                    file.print();
                }
            }
        }
    }
}
