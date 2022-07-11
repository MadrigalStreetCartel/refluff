use std::{io::{BufReader, Read}, path::PathBuf};
use binary_reader::Endian;
use clap::{Parser, Subcommand};

#[macro_use]
mod macros;
mod world;
mod strider;

#[derive(Debug, Subcommand)]
enum Commands {
    #[clap(name = "strider")]
    Strider {
        needle: u32,
        filename: PathBuf,
    },
    #[clap(name = "parse-world")]
    ParseWorld {
        filename: PathBuf,
    },
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        Commands::Strider { needle, filename } => {
            use colored::*;
            let data = {
                let file = std::fs::File::open(&filename)?;
                let mut reader = BufReader::new(file);
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf
            };
            let results = strider::Strider::new(needle, &data[..]).run()?;
            if !results.is_empty() {
                let filename_without_extension = filename
                    .file_name()
                    .map(|s| s.to_string_lossy())
                    .map(|s| s.to_owned())
                    .unwrap_or_default();
                println!("{}", filename_without_extension.white());
            }
            for result in results {
                let offset = format!("{: <6x}", result.offset).blue();
                let width = format!("u{}", result.width_bits).yellow();
                let endianness = match result.endian {
                    Endian::Little => "LE",
                    Endian::Big => "BE",
                    _ => ""
                }.green();
                let info = format!("({}; {})", width, endianness).bright_black();
                println!("| {}{} {}", "0x".black(), offset, info);
            }
        },
        Commands::ParseWorld { filename } => {
            let data = {
                let file = std::fs::File::open(filename)?;
                let mut reader = BufReader::new(file);
                let mut buf = Vec::new();
                reader.read_to_end(&mut buf)?;
                buf
            };
            let world = world::World::parse(&data[..])?;
            println!("{:?}", world);
        },
    }
    Ok(())
}
