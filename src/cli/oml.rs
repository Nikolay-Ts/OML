use clap::Parser;

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

    #[arg(short, long, default_value_t = 3u8)]
    depth: u8,

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