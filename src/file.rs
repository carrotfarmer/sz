pub struct File {
    file_name: String,
    file_size: u64,
}

impl File {
    pub fn new(file_name: String, file_size: u64) -> Self {
        Self {
            file_name,
            file_size,
        }
    }

    pub fn print(&self) {
        println!("{} ==> {}", self.file_name, self.file_size);
    }
}
