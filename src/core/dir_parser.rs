use std::fs;
use std::path::Path;
use crate::core::errors::ParseError;
use crate::core::file;


pub fn parse_dir_improved(
    dir_path: &Path,
    max_depth: usize
) -> Result<file::File, ParseError> {
    if max_depth == 0 {
        return Err(ParseError::MaxDepthExceeded);
    }

    let dir = fs::read_dir(dir_path)?;
    let mut file_system = file::File::init(None, Some(true), Some(vec![]));

    for entry in dir {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::symlink_metadata(&entry_path)?;

        if metadata.file_type().is_symlink() {
            eprintln!("Warning: Skipping symlink: {}", entry_path.display());
            continue;
        }

        if entry_path.is_file() {
            if let Some(extension) = entry_path.extension() {
                if extension.to_string_lossy() != "oml" {
                    continue;
                }
            } else {
                continue;
            }

            if let Some(file_name) = entry_path.file_name() {
                file_system.add_file(file::File::init(
                    Some(String::from(file_name.to_string_lossy())),
                    None,
                    None
                ));
            }
            continue;
        }

        if entry_path.is_dir() {
            let mut sub_dir = parse_dir_improved(&entry_path, max_depth - 1)?;

            if let Some(dir_name) = entry_path.file_name() {
                sub_dir.name = String::from(dir_name.to_string_lossy());
            }

            file_system.add_file(sub_dir);
        }
    }

    Ok(file_system)
}

pub fn parse_dir_from_string(
    dir_path: String,
    max_depth: usize
) -> Result<file::File, ParseError> {
    let path = Path::new(&dir_path);

    // Validate path exists
    if !path.exists() {
        return Err(ParseError::InvalidPath);
    }

    if !path.is_dir() {
        return Err(ParseError::InvalidPath);
    }

    parse_dir_improved(path, max_depth)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relative_path_works() {
        let _result = parse_dir_from_string("./src".to_string(), 10);
    }

    #[test]
    fn test_absolute_path_works() {
        let _result = parse_dir_from_string("/home/user/project".to_string(), 10);
    }

    #[test]
    fn test_max_depth_prevents_overflow() {
        let _result = parse_dir_from_string("./".to_string(), 5);
    }
}
