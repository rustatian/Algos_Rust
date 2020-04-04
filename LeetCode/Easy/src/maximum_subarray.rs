// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//
// Example:
//
// Input: [-2,1,-3,4,-1,2,1,-5,4],
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
//
// Follow up:
//
// If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.

use std::borrow::{Borrow, BorrowMut};

pub struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut max_sum = nums[0];
        let mut nums_mut = nums;
        let mut i = 1;

        loop {
            if i == n {
                break
            }
            if nums_mut[i - 1] > 0 {
                nums_mut[i] += nums_mut[i - 1]
            }
            max_sum = nums_mut[i].max(max_sum);
            i += 1;
        }
        max_sum
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}