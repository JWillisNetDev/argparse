use std::{fs::File, io::{BufReader, BufRead}, path::PathBuf};
use anyhow::{bail};
use chrono::{DateTime, FixedOffset};
use clap::{arg, command, Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple CLI to read a file")]
struct Cli {
    #[arg(short, long, help = "The path to the file to read")]
    file: PathBuf,
    #[command(subcommand)]
    command: Command
}

#[derive(Debug, Subcommand)]
enum Command {
    Show,
    Filter(FilterArgs),
}

// parsearg <File> filter --timestamp 2021-01-01T00:00:00+00:00

#[derive(Args, Debug)]
struct FilterArgs {
    #[arg(long, help = "The timestamp to filter by")]
    timestamp: Option<DateTime<FixedOffset>>,
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

    match cli.command {
        Command::Show => {
            for line in lines {
                println!("{}", line);
            }
        },
        Command::Filter => {
            
        }
    }

    Ok(())
}
