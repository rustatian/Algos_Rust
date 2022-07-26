struct Solution {}

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (f64::sqrt((2.0 * n as f64) + 0.25) - 0.5) as i32
    }
}

mod tests {
    use crate::arranging_coins::Solution;

    #[test]
    fn test() {
        assert_eq!(65535, Solution::arrange_coins(2147483647));
        assert_eq!(1, Solution::arrange_coins(2));
        assert_eq!(1, Solution::arrange_coins(1));
        assert_eq!(2, Solution::arrange_coins(3));
        assert_eq!(3, Solution::arrange_coins(8));
        assert_eq!(2, Solution::arrange_coins(5));
    }
}
