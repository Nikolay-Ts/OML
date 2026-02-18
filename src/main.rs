mod core;
mod cli;
mod generators;

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

    let oml_files = match cli.get_files() {
        Ok(files) => files,
        Err(e) => {
            eprintln!("An error was encountered when parsing the input files: {:?}", e);
            return;
        }
    };

    if oml_files.is_empty() {
        eprintln!("No .oml files found");
        return;
    }

    let generators = cli.get_generators();

    if generators.is_empty() {
        eprintln!("No language flag specified (e.g. --cpp)");
        return;
    }

    let output_dir = Path::new(&cli.output);

    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create output directory '{}': {}", cli.output, e);
        return;
    }

    for oml_file in &oml_files {
        for generator in &generators {
            for object in &oml_file.objects {
                let output_name = &object.name;

                match generator.generate(object, &oml_file.file_name) {
                    Ok(content) => {
                        let output_path = output_dir.join(
                            format!("{}.{}", output_name, generator.extension())
                        );
                        if let Err(e) = fs::write(&output_path, &content) {
                            eprintln!("Failed to write {}: {}", output_path.display(), e);
                        } else {
                            println!("Generated {}", output_path.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to generate {} for {}: {}", generator.extension(), output_name, e);
                    }
                }
            }
        }
    }
}
