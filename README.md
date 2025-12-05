# Advent of Code 2025

Solutions to AoC2025 puzzles in Rust. Uses the [aoc-runner](https://crates.io/crates/aoc-runner) crate for running and benching solutions.

# Example usage

```sh
# copy your session cookie from the AoC website
$ cargo aoc credentials {token}

# run the solution for the latest day
$ cargo aoc

# run the solution for a specific day and part
$ cargo aoc --day 5 --part 2

# run tests against example inputs
$ cargo test

# run benchmarks
$ cargo aoc bench
```