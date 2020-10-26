// Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
//
// (i.e., [0,1,2,4,5,6,7] might become [4,5,6,7,0,1,2]).
//
// You are given a target value to search. If found in the array return its index, otherwise return -1.
//
// You may assume no duplicate exists in the array.
//
// Your algorithm's runtime complexity must be in the order of O(log n).
//
// Example 1:
//
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
//
// Example 2:
//
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1
//

struct Solution {}

impl Solution {
    #[allow(clippy::comparison_chain)]
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut l = 0;
        let mut r = (nums.len() - 1) as i32;

        while l <= r {
            let m = l + (r - l) / 2;
            if nums[m as usize] == target {
                return m;
            } else if nums[m as usize] >= nums[l as usize] {
                if target >= nums[l as usize] && target < nums[m as usize] {
                    r = m - 1;
                } else {
                    l = m + 1;
                }
            } else if target <= nums[r as usize] && target > nums[m as usize] {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        -1
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
}