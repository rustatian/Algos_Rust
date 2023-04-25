struct Solution {}

impl Solution {
    pub fn majority_element(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() / 2]
    }
}

mod tests {
    use crate::majority_element::Solution;

    #[test]
    fn test() {
        assert_eq!(3, Solution::majority_element(vec![3, 2, 3]));
    }
}
