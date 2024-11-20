use std::{fs::File, io::{BufReader, BufRead}, path::PathBuf};
use anyhow::{bail};
use clap::{command, Parser, arg};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple CLI to read a file")]
struct Cli {
    #[arg(short, long, help = "The path to the file to read")]
    file: PathBuf
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let file = File::open(&cli.file)?;
    let reader = BufReader::new(file);

    let lines = reader.lines()
        .filter_map(|line| {
            if let Ok(line) = line {
                if line.is_empty() {
                    None
                } else {
                    Some(line)
                }
            } else {
                None
            }
        });

    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
