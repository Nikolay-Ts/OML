use std::io::Error;
use crate::core::file;
use std::fs;

///
/// recursively checks the directory path and its subdirectories for all .oml files and appends
/// them to the result keeping the hierarchy integrity.
///
pub fn parse_dir(dir_path: String) -> Result<file::File,  Error> {
    let dir =  fs::read_dir(dir_path);
    match dir {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    let mut file_system = file::File::init(None, Some(true), Some(vec![]));
    for entry in dir? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_file() {
            if let Some(extension) = entry_path.extension() {
                if extension.to_string_lossy() == "oml" {
                    continue
                }
            }

            if let Some(file_name) = entry_path.file_name(){
                file_system.add_file(file::File::init(
                    Some(String::from(file_name.to_string_lossy())),
                    None,
                    None
                ));
            }
            continue;
        }

        if entry_path.is_dir() {
            let sub_dir = parse_dir(String::from(entry_path.to_string_lossy()));
            match sub_dir {
                Ok(_) => {},
                Err(e) => return Err(e)
            }

            let mut sub_dir = sub_dir?;
            sub_dir.name = String::from(entry_path.to_string_lossy());

            file_system.add_file(sub_dir);
        }
    }

    Ok(file_system)
}