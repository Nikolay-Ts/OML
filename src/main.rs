mod core;
mod cpp;
mod cli;

use std::fs;
use std::path::Path;

use clap::Parser;
use cli::oml::OmlCli;
use crate::cpp::oml_cpp::oml_to_cpp;

fn main() {
    let cli = OmlCli::parse();

    if !cli.has_inputs() {
        OmlCli::print_help();
        return;
    }

    let objects = match cli.get_files() {
        Ok(objects) => objects,
        Err(e) => {
            eprintln!("An error was encountered when parsing the input files: {:?}", e);
            return;
        }
    };

    if objects.is_empty() {
        eprintln!("No .oml files found");
        return;
    }

    if cli.cpp {
        let output_dir = Path::new(&cli.output);

        if let Err(e) = fs::create_dir_all(output_dir) {
            eprintln!("Failed to create output directory '{}': {}", cli.output, e);
            return;
        }

        for object in &objects {
            let file_name = &object.file_name;

            match oml_to_cpp(object, file_name) {
                Ok(cpp_content) => {
                    let output_path = output_dir.join(format!("{}.h", file_name));
                    if let Err(e) = fs::write(&output_path, &cpp_content) {
                        eprintln!("Failed to write {}: {}", output_path.display(), e);
                    } else {
                        println!("Generated {}", output_path.display());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to generate C++ for {}: {}", file_name, e);
                }
            }
        }
    }
}
