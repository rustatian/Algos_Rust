struct Solution {}

impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        let count = high - low + 1;

        match count % 2 {
            // even
            0 => count / 2,
            // odd
            _ => count / 2 + ((low & 1) | (high & 1)),
        }
    }
}

mod tests {
    use crate::count_odds::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::count_odds(3, 7))
    }
}
