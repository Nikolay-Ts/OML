mod core;
mod cpp;
mod cli;

use clap::Parser;
use cli::oml::OmlCli;
use core::file::File;

fn main() {
    let cli = OmlCli::parse();
    let _ = File::init(None, None, None);

    println!("Hello, world!");
}
