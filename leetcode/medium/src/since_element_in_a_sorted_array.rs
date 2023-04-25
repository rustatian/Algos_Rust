struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut el = nums[0];

        for i in 1..nums.len() {
            el ^= nums[i]
        }

        return el;
    }
}

mod tests {
    use crate::since_element_in_a_sorted_array::Solution;

    #[test]
    fn test() {
        assert_eq!(
            2,
            Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8])
        );
    }
}
