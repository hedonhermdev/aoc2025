use std::ops::RangeInclusive;

use anyhow::{Context, Result};

type Id = usize;

#[derive(Debug)]
struct Inventory {
    ranges: Vec<RangeInclusive<Id>>,
}

impl Inventory {
    fn new(mut ranges: Vec<RangeInclusive<Id>>) -> Self {
        ranges.sort_by_key(|r| *r.start());
        
        let mut merged = vec![ranges[0].clone()];
        
        for range in ranges {
            if let Some(last) = merged.last_mut() {
                if range.start() <= last.end() {
                    *last = *last.start().min(range.start())..=*last.end().max(range.end());
                    continue;
                }
            }
            merged.push(range);          
        }

        Self { ranges: merged }
    }    
    
    fn is_fresh(&self, id: Id) -> bool {
        let mut lo = 0;
        let mut hi = self.ranges.len() - 1;
        
        while lo <= hi {
            let mid = (lo + hi) / 2;

            if self.ranges[mid].contains(&id) {
                return true;
            } else if id < *self.ranges[mid].start() {
                if let Some(h) = mid.checked_sub(1) {
                    hi = h;
                } else {
                    return false
                }
            } else {
                lo = mid + 1;
            }
        }
        
        return false;
    }
    
    fn fresh_id_count(&self) -> usize {
        let mut count = 0;
        
        for range in self.ranges.iter() {
            count += range.end() - range.start() + 1;
        }
        
        count
    }
}


fn parse_input(input: &str) -> Result<(Vec<RangeInclusive<Id>>, Vec<Id>)> {
    let mut ranges = vec![];
    let mut ids = vec![];

    let mut lines = input.lines();
    
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (lo, hi) = line.split_once("-").context("invalid range")?;
        let lo = lo.parse::<usize>()?;
        let hi = hi.parse::<usize>()?;
        
        ranges.push(lo..=hi);
    }
    
    while let Some(line) = lines.next() {
        ids.push(line.parse::<usize>()?);
    }
    
    Ok((ranges, ids))
}

pub fn puzzle1(input: &str) -> anyhow::Result<usize> {
    let (ranges, ids) = parse_input(input)?;
    
    let inventory = Inventory::new(ranges);
    
    let mut count = 0;
    
    for id in ids {
        if inventory.is_fresh(id) {
            count += 1;
        }
    }
    
    Ok(count)
}

pub fn puzzle2(input: &str) -> anyhow::Result<usize> {
    let (ranges, _) = parse_input(input)?;
    
    let inventory = Inventory::new(ranges);
    dbg!(&inventory);
    
    Ok(inventory.fresh_id_count())
}


#[cfg(test)]
const TEST_INPUT: &str = r#"
3-5
10-14
16-20
12-18

1
5
8
11
17
32
"#;

crate::aoc_tests!(TEST_INPUT, 3, 14);