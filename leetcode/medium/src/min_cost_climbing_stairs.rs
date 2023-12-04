struct Solution {}

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len()];

        dp[cost.len() - 1] = 0;

        for i in 2..cost.len() - 1 {
            let min = std::cmp::min(dp[i - 1], dp[i - 2]);
            dp[i] = min + cost[i];
        }

        std::cmp::min(dp[0], dp[1])
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        // assert_eq!(
        //
        //     6,
        //     Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        // );
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
    }
}
