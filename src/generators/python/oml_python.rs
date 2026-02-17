use crate::core::oml_object::{
    OmlObject, ObjectType, Variable, VariableVisibility, VariableModifier
};
use crate::core::generate::Generate;
use std::error::Error;
use std::fmt::Write;

pub struct PythonGenerator {
    pub use_data_class: bool,
}
impl PythonGenerator {
    pub fn new(use_data_class: bool) -> Self {
        Self { use_data_class }
    }
}

impl Generate for PythonGenerator {
    fn generate(&self, oml_object: &OmlObject, file_name: &str) -> Result<String, Box<dyn Error>> {
        let mut py_file = String::from("");

        writeln!(py_file, "// This file has been generated from {}.oml", file_name)?;
        writeln!(py_file)?;

        match &oml_object.oml_type {
            ObjectType::ENUM => generate_enum(oml_object, &mut py_file)?,
            ObjectType::CLASS | ObjectType::STRUCT => {
                generate_class(oml_object, &mut py_file, self.use_data_class)?
            },
            ObjectType::UNDECIDED => return Err("Cannot generate code for UNDECIDED object type".into()),
        }


        Ok(py_file)
    }

    fn extension(&self) -> &str { "py" }
}

fn generate_enum(oml_object: &OmlObject, py_file: &mut String, ) -> Result<(), std::fmt::Error> {
    writeln!(py_file, "from enum import Enum")?;
    writeln!(py_file)?;

    writeln!(py_file, "class {}(Enum):", oml_object.name)?;

    for (index, var) in oml_object.variables.iter().enumerate() {
        writeln!(py_file, "{} = {}", var.name, index)?;
    }

    Ok(())
}

fn generate_class(
    oml_object: &OmlObject,
    py_file: &mut String,
    use_data_class: bool
) -> Result<(), std::fmt::Error> {

    Ok(())
}

fn generate_data_class(oml_object: &OmlObject, py_file: &mut String) -> Result<(), std::fmt::Error> {

    Ok(())
}



#[inline]
fn convert_type(var_type: &str) -> String {
    match var_type {
        "int8" => "int",
        "int16" => "int",
        "int32" => "int",
        "int64" => "int",
        "uint8" => "int",
        "uint16" => "int",
        "uint32" => "int",
        "uint64" => "int",
        "float" => "float",
        "double" => "float",
        "bool" => "bool",
        "string" => "str",
        "char" => "str",
        _ => ""
    }.to_string()
}