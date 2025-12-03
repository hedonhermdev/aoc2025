use std::{
    io::{Read, stdin},
    path::PathBuf,
};

use clap::Parser;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(short, long)]
    day: u8,

    #[arg(short, long)]
    puzzle: u8,

    #[arg()]
    input_file: Option<PathBuf>,
}
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut buf = Vec::new();

    match args.input_file {
        Some(ref path) => {
            std::fs::File::open(path)?.read_to_end(&mut buf)?;
        }
        None => {
            stdin().read_to_end(&mut buf)?;
        }
    };

    let input = String::from_utf8(buf)?;

    let result = aoc::solution::run_solution(args.day, args.puzzle, &input.trim())?;

    println!("{}", result);

    Ok(())
}
