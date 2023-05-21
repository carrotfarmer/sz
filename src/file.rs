use std::fmt::Display;

use crate::file_size::FileSize;

pub struct File {
    file_name: String,
    file_size: FileSize,
}

impl File {
    pub fn new(file_name: String, file_size: f64) -> Self {
        let file_size = FileSize::new(file_size);

        Self {
            file_name,
            file_size,
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("{} ==> {}", self.file_name, self.file_size);
        Ok(())
    }
}
