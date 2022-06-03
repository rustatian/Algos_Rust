/*
Given an array of integers nums which is sorted in ascending order, and an integer target, write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.

You must write an algorithm with O(log n) runtime complexity.



Example 1:

Input: nums = [-1,0,3,5,9,12], target = 9
Output: 4
Explanation: 9 exists in nums and its index is 4
Example 2:

Input: nums = [-1,0,3,5,9,12], target = 2
Output: -1
Explanation: 2 does not exist in nums so return -1


Time complexity: https://en.wikipedia.org/wiki/Master_theorem_(analysis_of_algorithms) (case 2), a=1,b=2,d=0
 */

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let pivot = left + (right - left) / 2;

            match nums.get(pivot) {
                None => {
                    return -1;
                }
                Some(val) => match val.cmp(&target) {
                    Ordering::Less => {
                        left = pivot + 1;
                    }
                    Ordering::Equal => {
                        return pivot as i32;
                    }
                    Ordering::Greater => {
                        right = pivot - 1;
                    }
                },
            };
        }

       -1
    }
}

#[cfg(test)]
mod tests {
    use crate::bin_search::Solution;

    #[test]
    fn test() {
        let v = vec![-1, 0, 3, 5, 9, 12];
        let res = Solution::search(v, 2);
        assert_eq!(res, -1);

        let v = vec![2, 5];
        let res = Solution::search(v, 2);
        assert_eq!(res, 0);
    }
}
