use anyhow::Result;
use aoc_runner_derive::*;

type Input = Vec<Vec<u8>>;

struct Bank<'a> {
    batteries: &'a [u8],
}

impl<'a> Bank<'a> {
    fn new(batteries: &'a [u8]) -> Self {
        Self {
            batteries
        }
    }

    fn joltage(&self, n_batteries: u8) -> u64 {
        let mut res: u64 = 0;
        let mut lo = 0;
        let mut hi = self.batteries.len() - n_batteries as usize;

        for _ in 0..n_batteries {
            let mut max = 0;
            let mut max_idx = 0;
            
            for j in lo..=hi {
                if self.batteries[j] > max {
                    max = self.batteries[j];
                    max_idx = j;
                }                 
            }
            
            res = res * 10 + max as u64;
            lo = max_idx + 1;
            hi += 1;
        }
        
        res
    }
}


#[aoc_generator(day3)]
fn parse_input(input: &str) -> Input {
    input
    .lines()
    .map(|line| {
        line.bytes().map(|b| b - b'0').collect()
    })
    .collect()
}

#[aoc(day3, part1)]
fn puzzle1(input: &Input) -> u64 {
    let n_batteries = 2;
    
    let mut joltage = 0;
    
    for row in input {
        let bank = Bank::new(row);
        
        joltage += bank.joltage(n_batteries);
    }
    
    joltage
}

#[aoc(day3, part2)]
pub fn puzzle2(input: &Input) -> u64 {
    
    let n_batteries = 12;
    
    let mut joltage = 0;
    
    for row in input {
        let bank = Bank::new(row);
        
        joltage += bank.joltage(n_batteries);
    }
    
    joltage
}

#[cfg(test)]
const TEST_INPUT: &str = r#"
987654321111111
811111111111119
234234234234278
818181911112111
"#;

crate::aoc_test!(TEST_INPUT, 357, 3121910778619);