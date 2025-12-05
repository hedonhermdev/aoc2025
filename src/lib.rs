pub mod solution;

#[macro_export]
macro_rules! aoc_tests {
    ($test_input:expr, $p1:expr, $p2:expr) => {
        #[cfg(test)]
        mod test {
            use super::*;

            #[test]
            fn test_puzzle1() {
                assert_eq!(puzzle1($test_input.trim()).unwrap(), $p1);
            }

            #[test]
            fn test_puzzle2() {
                assert_eq!(puzzle2($test_input.trim()).unwrap(), $p2);
            }
        }
    };
}
