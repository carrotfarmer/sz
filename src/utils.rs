use std::path::Path;

pub fn get_file_size(path: &Path) -> f64 {
    match path.metadata() {
        Ok(metadata) => metadata.len() as f64,
        Err(e) => {
            println!("sz: error while reading path: {}", e);
            0.0
        }
    }
}
