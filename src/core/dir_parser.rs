use std::fs;
use std::path::Path;
use crate::core::errors::ParseError;
use crate::core::oml_object::OmlObject;

pub fn parse_path(
    path: &Path,
    max_depth: usize
) -> Result<Vec<OmlObject>, ParseError> {
    if max_depth == 0 {
        return Err(ParseError::MaxDepthExceeded);
    }

    let mut result_object = vec![];

    let metadata = fs::symlink_metadata(path)?;

    if metadata.file_type().is_symlink() {
        return Err(ParseError::InvalidPath);
    }

    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension.to_string_lossy() != "oml" {
                return Err(ParseError::InvalidPath);
            }
        } else {
            return Err(ParseError::InvalidPath);
        }

        let file_name = path.file_name()
            .ok_or(ParseError::InvalidPath)?
            .to_string_lossy()
            .to_string();

        return match OmlObject::get_from_file(path) {
            Ok(oml) => {
                result_object.push(oml);
                Ok(result_object)
            },
            Err(_) => {
                eprintln!("couldt not obtain the oml object from file {}", file_name);
                Err(ParseError::InvalidPath)
            }
        }
    }

    if !path.is_dir() {
        return Err(ParseError::InvalidPath);
    }

    let dir = fs::read_dir(path)?;

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
                    eprintln!("Warning: Skipping non-oml file: {}", entry_path.display());
                    continue;
                }
            } else {
                continue;
            }

            match OmlObject::get_from_file(&entry_path) {
                Ok(oml) => result_object.push(oml),
                Err(e) => {
                    eprintln!("Warning: Failed to parse {}: {}", entry_path.display(), e);
                }
            }
            continue;
        }

        if entry_path.is_dir() {
            let mut sub_objects = parse_path(&entry_path, max_depth - 1)?;
            result_object.append(&mut sub_objects);
        }
    }

    Ok(result_object)
}

pub fn parse_dir_from_string(
    path_str: String,
    max_depth: usize
) -> Result<Vec<OmlObject>, ParseError> {
    let path = Path::new(&path_str);

    if !path.exists() {
        return Err(ParseError::InvalidPath);
    }

    parse_path(path, max_depth)
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
