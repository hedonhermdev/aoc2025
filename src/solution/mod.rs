use anyhow::Result;
use std::path::Path;

mod day1;
mod day2;
mod day3;
mod day4;

macro_rules! map_res {
    ($sol:expr) => {
        $sol.map(|res| res.to_string())
    };
}

pub fn run_solution(day: u8, puzzle: u8, input: &str) -> Result<String> {
    match (day, puzzle) {
        (1, 1) => map_res!(day1::puzzle1(input)),
        (1, 2) => map_res!(day1::puzzle2(input)),
        (2, 1) => map_res!(day2::puzzle1(input)),
        (2, 2) => map_res!(day2::puzzle2(input)),
        (3, 1) => map_res!(day3::puzzle1(input)),
        (3, 2) => map_res!(day3::puzzle2(input)),
        (4, 1) => map_res!(day4::puzzle1(input)),
        (4, 2) => map_res!(day4::puzzle2(input)),
        _ => panic!("Solution for day {} puzzle {} not implemented", day, puzzle),
    }
}
