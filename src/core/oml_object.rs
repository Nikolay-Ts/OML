use std::cmp::PartialEq;
use std::fmt;
use std::fmt::format;
use std::fs;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use regex::Regex;

use crate::core::errors;

#[derive(Debug, PartialEq)]
enum ObjectType {
    ENUM,
    CLASS,
    STRUCT,
    UNDECIDED
}
#[derive(Debug)]
enum VariableModifier {
    CONST,
    MUT,
    STATIC,
    OPTIONAL,
}

#[derive(Debug)]
enum VariableVisibility {
    PRIVATE,
    PUBLIC,
    PROTECTED
}

#[derive(Debug)]
struct Variable {
    pub var_mod: Vec<VariableModifier>,
    pub visibility: VariableVisibility,
    pub var_type: String
}

#[derive(Debug)]
pub struct OmlObject {
    pub oml_type: ObjectType,
    pub name: String,
    pub variables: Vec<Variable>
}

impl OmlObject {
    const CLASS_NAME: &'static str = "class";
    const ENUM_NAME: &'static str = "enum";
    const STRUCT_NAME: &'static str = "struct";

    fn new() -> Self {
        Self {
            oml_type: ObjectType::UNDECIDED,
            name: String::from(""),
            variables: vec![]
        }
    }

    pub fn get_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(file_path);
        let content = fs::read_to_string(path)?;

        let mut oml_object = Self {
            oml_type: ObjectType::UNDECIDED,
            name: String::from("Nothing"),
            variables: vec![]
        };

        // oml_object.scan_file(file_path)?;
        oml_object.scan_file(content)?;

        Ok(oml_object)
    }

    fn scan_file(&mut self, content: String) -> Result<(), Box<dyn std::error::Error>> {
        let lines = content.split(|c| c == ';' || c == '\n').collect::<Vec<_>>();
        for line in lines {
            let text = line.split(' ').collect::<Vec<_>>();

            if self.oml_type == ObjectType::UNDECIDED {
                match text[0] {
                    Self::CLASS_NAME => {
                        self.oml_type = ObjectType::CLASS;
                        self.assign_obj_name(text[1])?;
                    }
                    Self::ENUM_NAME => {
                        self.oml_type = ObjectType::ENUM;
                        self.assign_obj_name(text[1])?;
                    }
                    Self::STRUCT_NAME => {
                        self.oml_type = ObjectType::STRUCT;
                        self.assign_obj_name(text[1])?;
                    }
                    _ => {},
                }
            }
        }

        Ok(())
    }

    // todo implement the custom errors
    fn assign_obj_name(&mut self, name: &str) -> Result<(), errors::NameError> {
        match Self::is_valid_name(name) {
            true => self.name = name.to_string(),
            false => {
                let message = format!("{} is not a valid obj name.", name);
                return Err(errors::NameError::new(message));
            }
        }

        Ok(())
    }

    /// A name is correct if it begins with a letter (case-insensitive)
    /// and does not contain the following list of forbidden characters [/ \ | < > : " ? *].
    /// This is to ensure that the names can be translated to valid file, class/enum and variable
    /// names in other programing languages.
    #[inline]
    fn is_valid_name(name: &str) -> bool {
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9_.-]*$").unwrap();
        re.is_match(name)
    }
}


mod test {
    use super::*;

    const VALID_NAMES: [&str; 8] = [
        "myfile.txt",
        "variable_name",
        "Config",
        "test123",
        "my-file-name",
        "file.tar.gz",
        "a",
        "MyClass"
    ];

    const INVALID_NAMES: [&str; 8] = [
        "123file",
        "_private",
        "-file",
        "my file",
        "file@name",
        "my$var",
        "file/path",
        "",
    ];

    #[test]
    fn test_name_validity() {
        for valid_name in VALID_NAMES {
            assert_eq!(OmlObject::is_valid_name(valid_name), true);
        }

        for valid_name in INVALID_NAMES {
            assert_eq!(OmlObject::is_valid_name(valid_name), false);
        }
    }

    #[test]
    fn test_assign_name() {
        let mut oml_obj = OmlObject::new();

        for valid_name in VALID_NAMES {
            oml_obj.assign_obj_name(valid_name).expect("this should not happen");
            assert_eq!(oml_obj.name, valid_name);
        }

        for invalid_name in INVALID_NAMES {
            let error =   oml_obj.assign_obj_name(invalid_name).unwrap_err();
            let message = format!("{} is not a valid obj name.", invalid_name);
            assert_eq!(error.message, message);
        }

    }

}
