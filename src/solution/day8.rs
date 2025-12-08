use core::num;
use itertools::Itertools;
use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_runner_derive::*;

#[derive(PartialEq, Eq, Hash)]
struct JunctionBox(u64, u64, u64);

type Input = Vec<JunctionBox>;

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

struct DistPair<'a> {
    j1: &'a JunctionBox,
    j2: &'a JunctionBox,
    dist: f64,
}

impl<'a> DistPair<'a> {
    fn new(j1: &'a JunctionBox, j2: &'a JunctionBox) -> Self {
        let dx = (j1.0 as f64 - j2.0 as f64).powi(2) as f64;
        let dy = (j1.1 as f64 - j2.1 as f64).powi(2) as f64;
        let dz = (j1.2 as f64 - j2.2 as f64).powi(2) as f64;

        let dist = dx + dy + dz;

        Self { j1, j2, dist }
    }

    fn from_arr(arr: [&'a JunctionBox; 2]) -> Self {
        Self::new(arr[0], arr[1])
    }

    fn pair(&self) -> (&'a JunctionBox, &'a JunctionBox) {
        (self.j1, self.j2)
    }
}

impl<'a> PartialEq for DistPair<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl<'a> Eq for DistPair<'a> {}

impl<'a> PartialOrd for DistPair<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.dist.partial_cmp(&other.dist)
    }
}

impl<'a> Ord for DistPair<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[aoc(day8, part1)]
fn puzzle1(input: &Input) -> u64 {
    let mut heap = BinaryHeap::new();

    for [j1, j2] in input.iter().array_combinations() {
        heap.push(DistPair::new(j1, j2));

        if heap.len() > 1000 {
            heap.pop();
        }
    }

    let mut dsu = DSU::new(input);
    for dp in heap {
        let (j1, j2) = dp.pair();
        dsu.union(j1, j2);
    }

    dsu.roots()
        .map(|root| dsu.size[root] as u64)
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[aoc(day8, part2)]
fn puzzle2(input: &Input) -> u64 {
    let mut dsu = DSU::new(input);
    for (j1, j2) in input
        .iter()
        .array_combinations()
        .map(DistPair::from_arr)
        .sorted()
        .map(|dp| dp.pair())
    {
        dsu.union(j1, j2);
        if dsu.count() == 1 {
            return j1.0 * j2.0;
        }
    }

    unreachable!()
}
