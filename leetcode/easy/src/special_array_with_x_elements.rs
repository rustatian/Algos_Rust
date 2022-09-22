struct Solution {}

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        use std::cmp::Ordering;
        nums.sort_unstable();
        nums.reverse();

        let mut start = 0;
        let mut end = nums.len();

        while start < end {
            let middle = start + (end - start) / 2;
            match middle.cmp(&(nums[middle] as usize)) {
                Ordering::Less => {
                    start = middle + 1;
                }

                _ => {
                    end = middle;
                }
            }
        }

        // start can't be bigger (if we hit only Ordering::Less)
        // if start is equal to nums[start] than we finished but not reached slice to the end and
        // this is not possible that x exists.
        // e.g. [7, 6, 5, 3, 0] -> index 3, number 3 -> should be at least 3 numbers equal or less than 3
        // but we already have 3 numbers which are greater, and only 1 number to check.
        if start < nums.len() && start == nums[start] as usize {
            return -1;
        }

        start as i32
    }
}

mod tests {
    use crate::special_array_with_x_elements::Solution;

    #[test]
    fn test() {
        assert_eq!(-1, Solution::special_array(vec![3, 6, 5, 7, 0]));
        assert_eq!(2, Solution::special_array(vec![3, 5]));
        assert_eq!(-1, Solution::special_array(vec![0, 0]));
        assert_eq!(3, Solution::special_array(vec![0, 4, 3, 0, 4]));
    }
}
