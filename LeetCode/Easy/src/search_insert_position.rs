struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut pivot: i32 = 0;
        let mut left: i32 = 0;
        let mut right: i32 = (nums.len() - 1) as i32;

        while (left <= right) {
            pivot = left + (right - left) / 2;

            if nums[pivot as usize] == target {
                return pivot as i32;
            }

            if (target < nums[pivot as usize]) {
                right = pivot - 1;
            } else {
                left = pivot + 1;
            }
        }

        return left as i32;
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }
}
