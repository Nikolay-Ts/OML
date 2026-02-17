mod core;
mod cpp;
mod cli;

use std::fs;
use std::path::Path;

use clap::Parser;
use cli::oml::OmlCli;

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

    let generators= cli.get_generators();

    if generators.is_empty() {
        eprintln!("No language flag specified (e.g. --cpp)");
        return;
    }

    let output_dir = Path::new(&cli.output);

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory '{}': {}", cli.output, e);
        return;
    }

    for generator in &generators {
        for object in &objects {
            let file_name = &object.file_name;

            match generator.generate(object, file_name) {
                Ok(content) => {
                    let output_path = output_dir.join(
                        format!("{}.{}", file_name, generator.extension())
                    );
                    if let Err(e) = fs::write(&output_path, &content) {
                        eprintln!("Failed to write {}: {}", output_path.display(), e);
                    } else {
                        println!("Generated {}", output_path.display());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to generate {} for {}: {}", generator.extension(), file_name, e);
                }
            }
        }
    }
}
