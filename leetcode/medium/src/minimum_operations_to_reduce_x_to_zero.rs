struct Solution {}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, mut x: i32) -> i32 {
        use std::cmp::Ordering;

        let mut sum = 0;
        let mut steps = 0;

        for n in &nums {
            sum += *n;
        }

        let mut start = 0;
        let mut end = 0;
        let mut res = -1;
        let mut curr_sum = 0;

        while end < nums.len() {
            curr_sum += nums[end];

            while curr_sum > sum - x && start <= end {
                curr_sum -= nums[start];
                start += 1;
            }

            if curr_sum == sum - x {
                res = std::cmp::max(res, (end - start + 1) as i32);
            }

            end += 1;
        }

        if res == -1 {
            return -1;
        }

        nums.len() as i32 - res
    }
}

#[cfg(test)]
mod tests {
    use crate::minimum_operations_to_reduce_x_to_zero::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::min_operations(vec![1, 1, 4, 2, 3], 5));
    }
}
