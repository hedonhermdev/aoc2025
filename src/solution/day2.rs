use aoc_runner_derive::*;
use std::ops::RangeInclusive;

type Input = Vec<RangeInclusive<u64>>;

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Input {
    input
        .split(",")
        .map(|s| {
            let (start, end) = s.split_once("-").unwrap();

            let start = start.parse::<u64>().unwrap();
            let end = end.parse::<u64>().unwrap();

            start..=end
        })
        .collect()
}

fn is_invalid1(id: u64) -> bool {
    let digits = (id as f64).log10().floor() as u32 + 1;
    if !digits.is_multiple_of(2) {
        false
    } else {
        let div = 10u64.pow(digits / 2);
        let left = id / div;
        let right = id % div;
        left == right
    }
}

#[aoc(day2, part1)]
fn puzzle1(input: &Input) -> u64 {
    input
        .iter()
        .flat_map(|r| r.clone())
        .filter(|&id| is_invalid1(id))
        .sum()
}

fn is_invalid2(id: u64) -> bool {
    let n_digits = (id as f64).log10().floor() as u32 + 1;
    let last_n_digits = |id: u64, n: u32| id % (10u64.pow(n));

    let repeat_group = |group: u64, n_repeats: u32| {
        let mut res = 0u64;
        let n_digits = (group as f64).log10().floor() as u32 + 1;

        for i in 0..n_repeats {
            res += group * 10u64.pow(i * n_digits);
        }

        res
    };

    for k in 1..(n_digits / 2 + 1) {
        if n_digits.is_multiple_of(k) {
            let group = last_n_digits(id, k);

            if repeat_group(group, n_digits / k) == id {
                return true;
            }
        }
    }
    false
}

#[aoc(day2, part2)]
fn puzzle2(input: &Input) -> u64 {
    input
        .iter()
        .flat_map(|r| r.clone())
        .filter(|&id| is_invalid2(id))
        .sum()
}

#[cfg(test)]
const TEST_INPUT: &str = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"#;

crate::aoc_test!(TEST_INPUT, 1227775554, 4174379265);
