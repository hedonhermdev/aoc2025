use std::backtrace;

use anyhow::Result;

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
            hi = hi + 1;
        }
        
        res
    }
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
    .lines()
    .map(|line| {
        line.bytes().map(|b| b - b'0').collect()
    })
    .collect()
}

pub fn puzzle1(input: &str) -> Result<u64> {
    let input = parse_input(input);
    
    let n_batteries = 2;
    
    let mut joltage = 0;
    
    for row in input {
        let bank = Bank::new(&row);
        
        joltage += bank.joltage(n_batteries) as u64;
    }
    
    Ok(joltage)
}

pub fn puzzle2(input: &str) -> Result<u64> {
    let input = parse_input(input);
    
    let n_batteries = 12;
    
    let mut joltage = 0;
    
    for row in input {
        let bank = Bank::new(&row);
        
        joltage += bank.joltage(n_batteries) as u64;
    }
    
    Ok(joltage)
}

const TEST_INPUT: &str = r#"987654321111111
811111111111119
234234234234278
818181911112111
"#;

crate::aoc_tests!(TEST_INPUT, 357, 3121910778619);