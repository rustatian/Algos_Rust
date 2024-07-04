struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut pivot: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;

        while left <= right {
            pivot = left + (right - left) / 2;
            use std::cmp::Ordering;

            match nums[pivot as usize].cmp(&target) {
                Ordering::Less => {
                    left = pivot + 1;
                }
                Ordering::Equal => {
                    return pivot as i32;
                }
                Ordering::Greater => {
                    right = pivot - 1;
                }
            }
        }

        left as i32
    }
}

mod tests {
    use crate::search_insert_position::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }
}
