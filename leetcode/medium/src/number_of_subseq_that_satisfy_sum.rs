struct Solution {}

impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        1
    }
}

mod tests {
    use crate::number_of_subseq_that_satisfy_sum::Solution;

    #[test]
    fn test() {
        assert_eq!(4, Solution::num_subseq(vec![3, 5, 6, 7], 9));
    }
}
