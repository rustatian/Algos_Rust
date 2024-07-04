struct Solution {}

impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(1, Solution::min_increment_for_unique(vec![1, 2, 3]));
    }
}
