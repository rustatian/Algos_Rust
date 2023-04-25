struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        // C(n,k) = C(n-1,k-1) + C(n-1,k)
        /*
        In particular, look at the second number from the left in each row. Each of those has a one to its upper left, and to its upper right is the row number of the previous row. Therefore, their sum is one plus the previous row number, and the result is the row we are in.
        Another one of the numerous interesting Pascal's triangle patterns is its symmetry. Observe that in any row, if we read the consecutive numbers from the left, we will obtain the same as if we read them from the right. Is it magic or mathematics again? Well, let's try to understand what's happening here.
        By definition, the number on the k-th place in the n-th row depicts in how many ways we can pick k elements from a set of n elements. But what if instead, we pointed out the elements that we don't choose? It may sound a bit vague, so why don't we show you an example.
                 */
        for i in 0..num_rows as usize {
            // prefill the vector
            res.push(vec![1; i + 1]);
            for j in 1..i {
                res[i][j] = res[i - 1][j - 1] + res[i - 1][j];
            }
        }

        res
    }
}

mod tests {
    use crate::pascals_triangle::Solution;

    #[test]
    fn test() {
        let res = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
            vec![1, 5, 10, 10, 5, 1],
            vec![1, 6, 15, 20, 15, 6, 1],
        ];
        assert_eq!(res, Solution::generate(7));

        let res = vec![vec![1]];
        assert_eq!(res, Solution::generate(1));
    }
}
