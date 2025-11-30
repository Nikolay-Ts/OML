///
/// File is meant to represent a file structure. This will be used when parser a given directory
/// in order to keep the hierarchy integrity in the output path.
///
pub struct File {
    pub name: String,
    pub is_dir: bool,
    pub dir_files: Option<Vec<File>>,
}

impl File {
    pub fn init(name: Option<String>, is_dir: Option<bool>, dir_files: Option<Vec<File>> ) -> Self {
        File {
            name: name.unwrap_or(String::from("")),
            is_dir: is_dir.unwrap_or(false),
            dir_files
        }
    }

    pub fn add_file(&mut self, new_file: File) {
        if let Some(files) =self.dir_files.as_mut() {
            files.push(new_file);
        }
    }
}