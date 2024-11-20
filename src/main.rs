use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple CLI to read a file")]
struct Cli {
    file: PathBuf
}

fn main() {
    let cli = Cli::parse();

    println!("{}", cli.file.display());
}
