struct Solution {}

impl Solution {
    // bottom-up
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut h = vec![0; nums.len()];

        // base
        h[0] = nums[0];
        h[1] = std::cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            h[i] = std::cmp::max(h[i - 1], h[i - 2] + nums[i]);
        }

        // last
        h[nums.len() - 1]
    }
}

mod tests {
    use crate::house_robber::Solution;

    #[test]
    fn test() {
        assert_eq!(4, Solution::rob(vec![2, 1, 1, 2]));
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }
}
