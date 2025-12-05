pub mod solution;

#[macro_export]
macro_rules! aoc_test {
    ($test_input:expr, $p1:expr, $p2:expr) => {
        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn test_puzzle1() {
                let input = parse_input($test_input.trim());
                assert_eq!(puzzle1(&input), $p1);
            }

            #[test]
            fn test_puzzle2() {
                let input = parse_input($test_input.trim());
                assert_eq!(puzzle2(&input), $p2);
            }
        }
    };
}

aoc_runner_derive::aoc_lib!{ year = 2025 }