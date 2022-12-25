struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        // 100 - nums.len() max
        let mut memo = vec![-1; nums.len()];
        let res = Solution::from(0, &mut memo, &nums);
        println!("{}", res);
        res
    }

    pub fn from(idx: usize, memo: &mut Vec<i32>, houses: &Vec<i32>) -> i32 {
        if houses.len() <= idx {
            return 0;
        }

        if memo[idx] != -1 {
            return memo[idx];
        }

        memo[idx] = std::cmp::max(
            Solution::from(idx + 1, memo, houses),
            Solution::from(idx + 2, memo, houses) + houses[idx],
        );

        memo[idx]
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
