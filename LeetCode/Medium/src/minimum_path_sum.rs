// Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.
//
// Note: You can only move either down or right at any point in time.
//
// Example:
//
// Input:
// [
// [1,3,1],
// [1,5,1],
// [4,2,1]
// ]
// Output: 7
// Explanation: Because the path 1→3→1→1→1 minimizes the sum.

struct Solution {}

impl Solution {
    #[allow( clippy::assign_op_pattern)]
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = grid.clone();

        for (i, _) in grid.iter().enumerate().rev() {
            for (j, _) in grid[i].iter().enumerate().rev() {
                if i == grid.len() - 1 && j != grid[0].len() - 1 {
                    dp[i][j] = dp[i][j] + dp[i][j + 1];
                } else if j == grid[0].len() - 1 && i != grid.len() - 1 {
                    dp[i][j] = dp[i][j] + dp[i + 1][j];
                } else if j != grid[0].len() - 1 && i != grid.len() - 1 {
                    dp[i][j] = dp[i][j] + std::cmp::min(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }


        dp[0][0]
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 4, 8], vec![3, 2, 2, 4], vec![5, 7, 1, 9], vec![2, 3, 2, 3]]), 14);
    assert_eq!(Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]), 7);
}