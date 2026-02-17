use crate::core::oml_object::{
    OmlObject, ObjectType, Variable, VariableVisibility, VariableModifier
};
use crate::core::generate::Generate;
use std::error::Error;
use std::fmt::Write;
use crate::generators::kotlin::oml_kotlin::KotlinGenerator;

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
        todo!()
    }

    fn extension(&self) -> &str { "py" }
}