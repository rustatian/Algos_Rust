struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut res = vec![];

        // C(n,k) = C(n-1,k-1) + C(n-1,k)
        for i in 0..row_index as usize + 1 {
            res.push(vec![1; i + 1]);
            for j in 1..i {
                res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
            }
        }

        res[row_index as usize].clone()
    }
}

mod tests {
    use crate::pascals_triangle_2::Solution;

    #[test]
    fn test() {
        let res = vec![1, 3, 3, 1];
        assert_eq!(res, Solution::get_row(3));
    }
}
