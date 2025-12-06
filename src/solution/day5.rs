use std::ops::RangeInclusive;

use aoc_runner_derive::*;

type Id = usize;
type Input = (Vec<RangeInclusive<Id>>, Vec<Id>);

#[derive(Debug)]
struct Inventory {
    ranges: Vec<RangeInclusive<Id>>,
}

impl Inventory {
    fn new(mut ranges: Vec<RangeInclusive<Id>>) -> Self {
        ranges.sort_by_key(|r| *r.start());

        let mut merged = vec![ranges[0].clone()];

        for range in ranges {
            if let Some(last) = merged.last_mut()
                && range.start() <= last.end()
            {
                *last = *last.start().min(range.start())..=*last.end().max(range.end());
                continue;
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
                    return false;
                }
            } else {
                lo = mid + 1;
            }
        }

        false
    }

    fn fresh_id_count(&self) -> usize {
        let mut count = 0;

        for range in self.ranges.iter() {
            count += range.end() - range.start() + 1;
        }

        count
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Input {
    let mut ranges = vec![];
    let mut ids = vec![];

    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let (lo, hi) = line.split_once("-").unwrap();
        let lo = lo.parse::<usize>().unwrap();
        let hi = hi.parse::<usize>().unwrap();

        ranges.push(lo..=hi);
    }

    for line in lines {
        ids.push(line.parse::<usize>().unwrap());
    }

    (ranges, ids)
}

#[aoc(day5, part1)]
pub fn puzzle1(input: &Input) -> usize {
    let (ranges, ids) = input;

    let inventory = Inventory::new(ranges.clone());

    let mut count = 0;

    for id in ids {
        if inventory.is_fresh(*id) {
            count += 1;
        }
    }

    count
}

#[aoc(day5, part2)]
pub fn puzzle2(input: &Input) -> usize {
    let (ranges, _) = input;

    let inventory = Inventory::new(ranges.clone());

    inventory.fresh_id_count()
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

crate::aoc_test!(TEST_INPUT, 3, 14);
