struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        use std::cmp::Ordering;
        let mut start = 0;
        let mut end = numbers.len() - 1;

        while start <= end {
            let s_num = numbers[start];
            let e_num = numbers[end];

            match (s_num + e_num).cmp(&target) {
                Ordering::Less => {
                    start += 1;
                }
                // according to the description, we have this case for sure
                Ordering::Equal => {
                    return vec![1 + start as i32, 1 + end as i32];
                }
                Ordering::Greater => {
                    end -= 1;
                }
            }
        }

        vec![]
    }
}

mod tests {
    use crate::two_sum_2_input_array_sorted::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![1, 3], Solution::two_sum(vec![2, 3, 4], 6));
        assert_eq!(vec![1, 2], Solution::two_sum(vec![-1, 0], -1));
    }
}
