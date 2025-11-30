use crate::File;
use std::fs;
use std::io::{BufReader, Error};
use std::io::prelude::*;

enum ObjectType {
    ENUM,
    CLASS
}

enum VariableModifier {
    CONST,
    MUT,
    STATIC,
    OPTIONAL,
}

enum VariableVisibility {
    PRIVATE,
    PUBLIC,
    PROTECTED
}

struct Variable {
    pub var_mod: Vec<VariableModifier>,
    pub visibility: VariableVisibility,
    pub var_type: String
}

struct OmlObject {
    pub oml_type: ObjectType,
    pub name: String,
    pub variables: Vec<Variable>
}

impl OmlObject {
    pub fn get_obj_from_file(file_path: &File) -> Result<Self, Error> {
        let mut oml_object = Self {
            oml_type: ObjectType::CLASS,
            name: String::from(""),
            variables: vec![]
        };

        let file = fs::File::open(file_path.name.clone())?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        
        todo!("read through the file and give it the proper attributes; each in its own function");
        
        Ok(oml_object)
    }
}