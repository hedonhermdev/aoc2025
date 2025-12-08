use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use aoc_runner_derive::*;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord)]
struct JunctionBox(u64, u64, u64);

type Input = Vec<JunctionBox>;

#[cfg(not(test))]
const NUM_CONNECTIONS: usize = 1000;

struct DSU<'a> {
    parent: HashMap<&'a JunctionBox, &'a JunctionBox>,
    size: HashMap<&'a JunctionBox, usize>,
    roots: HashSet<&'a JunctionBox>,
}

impl<'a> DSU<'a> {
    fn new(boxes: &'a [JunctionBox]) -> Self {
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut roots = HashSet::new();

        for j in boxes {
            parent.insert(j, j);
            size.insert(j, 1);
            roots.insert(j);
        }

        Self {
            parent,
            size,
            roots,
        }
    }

    fn union(&mut self, x: &'a JunctionBox, y: &'a JunctionBox) {
        let x = self.find(x);
        let y = self.find(y);

        if x != y {
            let size_x = self.size[x];
            let size_y = self.size[y];

            if size_x < size_y {
                self.parent.insert(x, y);
                self.size.insert(y, size_x + size_y);
                self.roots.insert(y);
                self.roots.remove(x);
            } else {
                self.parent.insert(y, x);
                self.size.insert(x, size_x + size_y);
                self.roots.insert(x);
                self.roots.remove(y);
            }
        }
    }

    fn find(&mut self, x: &'a JunctionBox) -> &'a JunctionBox {
        if self.parent[x] != x {
            let p = self.find(self.parent[x]);
            self.parent.insert(x, p);
        }
        self.parent[x]
    }

    fn roots(&self) -> impl Iterator<Item = &&'a JunctionBox> {
        self.roots.iter()
    }

    fn count(&self) -> usize {
        self.roots.len()
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let nums = line
                .split(',')
                .map(|num| num.trim().parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            JunctionBox(nums[0], nums[1], nums[2])
        })
        .collect()
}

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct DistPair<'a>(u128, &'a JunctionBox, &'a JunctionBox);

impl<'a> DistPair<'a> {
    fn new(j1: &'a JunctionBox, j2: &'a JunctionBox) -> Self {
        let dx = (j1.0 as i128 - j2.0 as i128).pow(2) as u128;
        let dy = (j1.1 as i128 - j2.1 as i128).pow(2) as u128;
        let dz = (j1.2 as i128 - j2.2 as i128).pow(2) as u128;

        let dist = dx + dy + dz;

        Self(dist, j1, j2)
    }

    fn from_arr(arr: [&'a JunctionBox; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    fn pair(&self) -> (&'a JunctionBox, &'a JunctionBox) {
        (self.1, self.2)
    }
}

#[aoc(day8, part1)]
fn puzzle1(input: &Input) -> u64 {
    let mut dsu = DSU::new(input);

    input.iter().array_combinations().map(|arr| DistPair::from_arr(arr)).k_largest(NUM_CONNECTIONS).for_each(|dp| {
        let (j1, j2) = dp.pair();
        dsu.union(j1, j2);        
    });

    dsu.roots()
        .map(|root| dsu.size[root] as u64)
        .k_largest(3)
        .product()
}

#[aoc(day8, part2)]
fn puzzle2(input: &Input) -> u64 {
    let mut dsu = DSU::new(input);

    for (j1, j2) in input
        .iter()
        .array_combinations()
        .map(DistPair::from_arr)
        .sorted_unstable()
        .map(|dp| dp.pair())
    {
        dsu.union(j1, j2);
        if dsu.count() == 1 {
            return j1.0 * j2.0;
        }
    }

    unreachable!()
}

#[cfg(test)]
const NUM_CONNECTIONS: usize = 10;

#[cfg(test)]
const TEST_INPUT: &str = r#"
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

crate::aoc_test!(TEST_INPUT, 40, 25272);
