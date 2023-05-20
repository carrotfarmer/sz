use std::path::Path;

pub fn get_file_size(path: &Path) -> u64 {
    match path.metadata() {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0
        }
    } 
}
