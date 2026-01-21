use clap::Parser;
use crate::core::errors;
use crate::core::dir_parser::parse_dir_from_string;
use crate::core::file;

#[derive(Parser)]
#[command(name = "oml")]
#[command(about = "Parse OML files and generate C++ headers", long_about = None)]
pub struct OmlCli {

    // names of the directories / oml files
    inputs: Option<Vec<String>>,

    #[arg(short, long, default_value = "./oml_output")]
    output: String,

    // if oml should check files within folders recursively
    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long, default_value_t = 3)]
    depth: usize,

    // language conversions

    #[arg(long)]
    cpp: bool,

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
    pub fn get_files(self) -> Result<Vec<file::File>, errors::ParseError> {
        if self.inputs.is_none() {
            return Err(errors::ParseError::InvalidPath);
        }

        let input_files = self.inputs.unwrap();
        let mut files = Vec::with_capacity(input_files.len());

        for file_name in input_files {
            let file = parse_dir_from_string(file_name.clone(), self.depth)?;
            files.push(file);
        }

        Ok(files)
    }
}