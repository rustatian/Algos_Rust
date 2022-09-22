/*
Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.



Example 1:

Input: nums = [1,2,3,1]
Output: true
Example 2:

Input: nums = [1,2,3,4]
Output: false
Example 3:

Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true
 */

use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return false;
        }

        let mut ht: HashMap<i32, bool> = HashMap::new();

        for i in nums {
            match ht.entry(i) {
                Entry::Occupied(_) => {
                    return true;
                }
                Entry::Vacant(vac) => {
                    vac.insert(true);
                }
            }
        }

        false
    }

    pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return false;
        }

        let ht: HashSet<i32> = HashSet::from_iter(nums.clone());

        ht.len() != nums.len()
    }
}

mod tests {
    use crate::contains_duplicates::Solution;

    #[test]
    fn test() {
        assert!(Solution::contains_duplicate2(vec![2, 14, 18, 22, 22]));
    }
}
