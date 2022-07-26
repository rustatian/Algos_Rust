struct Solution {}

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in grid {
            for (idx, j) in i.iter().enumerate() {
                if *j < 0 {
                    count += i.len() - idx;
                    break;
                }
            }
        }
        count as i32
    }
}

mod tests {
    use crate::count_neg_numbers::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::count_negatives(vec![vec![3, 2,], vec![1, 0],]));

        assert_eq!(
            8,
            Solution::count_negatives(vec![
                vec![4, 3, 2, -1],
                vec![3, 2, 1, -1],
                vec![1, 1, -1, -2],
                vec![-1, -1, -2, -3]
            ])
        );
    }
}
