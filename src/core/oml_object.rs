use std::cmp::PartialEq;
use std::fs;
use std::io::{BufReader, Error, Read};
use std::path::Path;

#[derive(Debug, PartialEq)]
enum ObjectType {
    ENUM,
    CLASS,
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

    pub fn get_from_file(file_path: &str) -> Result<Self, Error> {
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

    fn scan_file(&mut self, content: String) -> Result<(), Error> {
        let lines = content.split(|c| c == ';' || c == '\n').collect::<Vec<_>>();
        for line in lines {
            let text = line.split(' ').collect::<Vec<_>>();

            if self.oml_type == ObjectType::UNDECIDED {
                match text[0] {
                    Self::CLASS_NAME => {
                        self.oml_type = ObjectType::CLASS;
                        self.name = text[1].to_string();
                        continue;
                    }
                    Self::ENUM_NAME => {
                        self.oml_type = ObjectType::ENUM;
                        self.name = text[1].to_string();
                        continue;
                    }
                    _ => {},
                }
            }
        }

        Ok(())
    }

    /// A name is correct if it begins with a letter (case-insensitive)
    /// and does not contain the following list of forbidden characters [/ \ | < > : " ? *].
    /// This is to ensure that the names can be translated to valid file, class/enum and variable
    /// names in other programing languages.
    fn is_valid_name(name: &str) -> bool {
        todo!(a reg exp to filer this and figure out how to have a coressponding error message);
        
        true 
    }
}