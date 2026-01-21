mod core;
mod cpp;
mod cli;

use clap::Parser;
use cli::oml::OmlCli;

fn main() {
    let cli = OmlCli::parse();

    let files = match cli.get_files() {
        Ok(files) => files,
        Err(e) =>  {
         eprintln!("An error was encountered when parsing the input files: {:?}", e);
            return
        }
    };

    for file in files {
        println!("file has name {}", file.name);
    }

    println!("Hello, world!");
}
