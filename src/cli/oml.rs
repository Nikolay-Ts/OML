use clap::{Parser, CommandFactory};
use crate::core::errors;
use crate::core::dir_parser::parse_dir_from_string;
use crate::core::generate::Generate;
use crate::core::oml_object::OmlObject;
use crate::cpp::oml_cpp::CppGenerator;

#[derive(Parser)]
#[command(name = "oml")]
#[command(about = "Parse OML files and generate code from .oml definitions", long_about = None)]
pub struct OmlCli {

    // names of the directories / oml files
    inputs: Option<Vec<String>>,

    #[arg(short, long, default_value = "./oml_output")]
    pub output: String,

    // if oml should check files within folders recursively
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    // language conversions

    #[arg(long)]
    pub cpp: bool,

    #[arg(long)]
    python: bool,

    #[arg(long)]
    java: bool,

    #[arg(long)]
    kotlin: bool,

    #[arg(long)]
    rust: bool,

    #[arg(long)]
    typescript: bool,
}

impl OmlCli {
    pub fn has_inputs(&self) -> bool {
        self.inputs.is_some()
    }

    pub fn print_help() {
        Self::command().print_help().unwrap();
        println!();
    }

    pub fn get_files(&self) -> Result<Vec<OmlObject>, errors::ParseError> {
        let input_files = match &self.inputs {
            Some(inputs) => inputs,
            None => {
                return Err(errors::ParseError::InvalidPath);
            }
        };

        let mut files = Vec::new();

        for file_name in input_files {
            let mut parsed = parse_dir_from_string(file_name.clone(), self.depth)?;
            files.append(&mut parsed);
        }

        Ok(files)
    }

    pub fn get_generators(&self) -> Vec<Box<dyn Generate>> {
        let mut generators: Vec<Box<dyn Generate>> = Vec::new();

        if self.cpp {
            generators.push(Box::new(CppGenerator));
        }

        // TODO: add other generators as they are implemented
        // if self.python { generators.push(Box::new(PythonGenerator)); }
        // if self.java { generators.push(Box::new(JavaGenerator)); }
        // if self.kotlin { generators.push(Box::new(KotlinGenerator)); }
        // if self.rust { generators.push(Box::new(RustGenerator)); }
        // if self.typescript { generators.push(Box::new(TypescriptGenerator)); }

        generators
    }
}