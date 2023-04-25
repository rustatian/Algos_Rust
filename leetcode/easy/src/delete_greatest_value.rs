struct Solution {}

impl Solution {
    pub fn delete_greatest_value(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        let l = grid.len();

        for i in 0..l {
            grid[i].sort_unstable();
        }

        for i in 0..grid[0].len() {
            let mut m = i32::MIN;

            for j in 0..grid.len() {
                m = std::cmp::max(m, grid[j][i]);
            }

            sum += m;
        }

        sum
    }
}

mod tests {
    use crate::delete_greatest_value::Solution;

    #[test]
    fn test() {
        assert_eq!(
            8,
            Solution::delete_greatest_value(vec![vec![1, 2, 4], vec![3, 3, 1]])
        );
    }
}
