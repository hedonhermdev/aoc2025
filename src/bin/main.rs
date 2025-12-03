use std::{io::{Read, stdin}, path::PathBuf};

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    puzzle: u8,

    #[arg(default_value = "input.txt")]
    input_file: Option<PathBuf>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let input = match args.input_file {
        Some(ref path) => {
            std::fs::read_to_string(path)?
        }
        None => {
            let mut input = Vec::new();
            stdin().read_to_end(&mut input)?;
            String::from_utf8(input)?
        }
    };

    let result = aoc::solution::run_solution(args.day, args.puzzle, &input)?;

    println!("{}", result);

    Ok(())
}