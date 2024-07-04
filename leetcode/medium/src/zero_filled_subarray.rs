struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let mut counter: i64 = 0;
        let mut sum_of_1s: i64 = 0;

        for i in nums {
            if i == 0 {
                counter += 1;
            } else {
                counter = 0
            }
            sum_of_1s += counter;
        }

        sum_of_1s
    }
}

mod tests {
    use crate::zero_filled_subarray::Solution;

    #[test]
    fn test1() {
        assert_eq!(9, Solution::zero_filled_subarray(vec![0, 0, 0, 2, 0, 0]));
    }
}
