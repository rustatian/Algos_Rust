// Given an array nums, write a function to move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Example:
//
// Input: [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Note:
//
// You must do this in-place without making a copy of the array.
// Minimize the total number of operations.

pub struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        if nums.len() == 1 {
            return;
        }
        let mut non_zero = 0;
        let mut idx: usize = 0;

        // If the current element is not 0, then we need to
        // append it just in front of last non 0 element we found.
        loop {
            if idx == nums.len() {
                break;
            }
            if nums[idx] != 0 {
                nums[non_zero] = nums[idx];
                non_zero += 1;
            }
            idx += 1;
        }

        // operations table, have an input 0 1 0 3 12
        // actions:
        // value 0: idx = 0, nz (non_zero) = 0 --> 0 1 0 3 12
        // value 1: idx = 1, nz = 1 -->            1 1 0 3 12
        // value 0: idx = 2, nz = 1 -->            1 1 0 3 12
        // value 3: idx = 3, nz = 2 -->            1 3 0 3 12
        // value 12: idx = 4, nz = 3 -->           1 3 12 3 12
        // end: nz = 4

        // we know, that non_zero that should be 0 was at index 4
        // and we fill up that elements with 0
        for i in non_zero..nums.len() {
            nums[i] = 0;
        }
    }
}

#[test]
fn tests() {
    let mut v = vec![0, 1, 0, 3, 12];
    Solution::move_zeroes(&mut v);
    assert_eq!(v, vec![1, 3, 12, 0, 0]);
}

#[test]
fn tests_2() {
    let mut v = vec![1, 0];
    Solution::move_zeroes(&mut v);
    assert_eq!(v, vec![1, 0]);
}

#[test]
fn tests_3() {
    let mut v = vec![1, 0, 0];
    Solution::move_zeroes(&mut v);
    assert_eq!(v, vec![1, 0, 0]);
}