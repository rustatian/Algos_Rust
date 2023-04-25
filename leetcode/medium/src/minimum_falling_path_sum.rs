struct Solution {}

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let mut sum = i32::MAX;
        let mut memo = Vec::with_capacity(matrix.len());

        for _ in 0..memo.capacity() {
            memo.push(vec![i32::MIN; matrix[0].len()])
        }

        for st in 0..matrix.len() {
            sum = std::cmp::min(sum, Solution::helper(&matrix, &mut memo, 0, st as i32));
        }

        sum
    }

    fn helper(matrix: &Vec<Vec<i32>>, memo: &mut Vec<Vec<i32>>, row: i32, col: i32) -> i32 {
        if col < 0 || col == matrix.len() as i32 {
            return i32::MAX;
        }

        if row == matrix.len() as i32 - 1 {
            return matrix[row as usize][col as usize];
        }

        if memo[row as usize][col as usize] != i32::MIN {
            return memo[row as usize][col as usize];
        }

        let l = Solution::helper(matrix, memo, row + 1, col);
        let r = Solution::helper(matrix, memo, row + 1, col + 1);
        let c = Solution::helper(matrix, memo, row + 1, col - 1);

        memo[row as usize][col as usize] =
            std::cmp::min(l, std::cmp::min(c, r)) + matrix[row as usize][col as usize];
        memo[row as usize][col as usize]
    }
}

mod tests {
    use crate::minimum_falling_path_sum::Solution;

    #[test]
    fn test() {
        assert_eq!(
            -36,
            Solution::min_falling_path_sum(vec![
                vec![100, -42, -46, -41],
                vec![31, 97, 10, -10],
                vec![-58, -51, 82, 89],
                vec![51, 81, 69, -51]
            ])
        );
        assert_eq!(-48, Solution::min_falling_path_sum(vec![vec![-48]]));
        assert_eq!(
            13,
            Solution::min_falling_path_sum(vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]])
        );
    }
}
