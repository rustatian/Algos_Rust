struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let total = (n + 1) / 2;

        let mut res: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

        for i in 0..n {
            let v = vec![0; n as usize];
            res.push(v);
        }

        let mut value = 1;

        for start in 0..total {
            for row1 in start..n - start {
                res[start as usize][row1 as usize] = value;
                value += 1;
            }

            for col1 in start + 1..n - start {
                res[col1 as usize][(n - start - 1) as usize] = value;
                value += 1;
            }

            let mut idx = n - start - 2;
            while idx >= start {
                res[(n - start - 1) as usize][idx as usize] = value;
                value += 1;
                idx -= 1;
            }

            let mut idx_up = n - start - 2;
            while idx_up > start {
                res[idx_up as usize][start as usize] = value;
                idx_up -= 1;
                value += 1;
            }
        }

        res
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3)
        );
    }
}
