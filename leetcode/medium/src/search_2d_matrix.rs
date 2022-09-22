struct Solution {}

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        use std::cmp::Ordering;

        let size = matrix.len();
        if size == 0 {
            return false;
        }

        let first = matrix[0].len();

        let mut start = 0;
        // total number of elements
        let mut end = size * first - 1;

        while start <= end {
            let middle = start + (end - start) / 2;
            let middle_val = matrix[middle / first][middle % first];

            match middle_val.cmp(&target) {
                Ordering::Less => {
                    if middle == 0 {
                        return false;
                    }
                    start = middle + 1;
                }
                Ordering::Equal => return true,
                Ordering::Greater => {
                    if middle == 0 {
                        return false;
                    }
                    end = middle - 1;
                }
            }
        }

        false
    }

    pub fn search_matrix2(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for v in &matrix {
            // found a row
            if (*v)[0] <= target && (*v)[(*v).len() - 1] >= target {
                use std::cmp::Ordering;
                let mut start = 0;
                let mut end = v.len() - 1;

                while start <= end {
                    let middle = start + (end - start) / 2;

                    match v[middle].cmp(&target) {
                        Ordering::Less => {
                            start = middle + 1;
                        }
                        Ordering::Equal => return true,
                        Ordering::Greater => {
                            end = middle - 1;
                        }
                    }
                }
            }
        }

        false
    }
}

mod tests {
    use crate::search_2d_matrix::Solution;

    #[test]
    fn test() {
        assert!(Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3
        ));
        assert!(!Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13
        ));

        assert!(Solution::search_matrix(vec![vec![1]], 1));
        assert!(!Solution::search_matrix(vec![vec![1]], 0));
    }
}
