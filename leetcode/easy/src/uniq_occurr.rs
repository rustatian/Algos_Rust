/*
Given an array of integers arr, return true if the number of occurrences of each value in the array is unique, or false otherwise.



Example 1:

Input: arr = [1,2,2,1,1,3]
Output: true
Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
Example 2:

Input: arr = [1,2]
Output: false
Example 3:

Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
Output: true
 */

use std::collections::hash_map::Entry;
use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let mut hm: HashMap<usize, usize> = HashMap::new();

        for i in arr {
            match hm.entry(i as usize) {
                Entry::Occupied(mut val) => {
                    *val.get_mut() += 1;
                }
                Entry::Vacant(val) => {
                    val.insert(1);
                }
            }
        }

        let mut hm2 = HashMap::new();
        for (_, v) in hm {
            match hm2.entry(v as usize) {
                Entry::Occupied(_) => return false,
                Entry::Vacant(val) => {
                    val.insert(v);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::uniq_occurr::Solution;

    #[test]
    fn test1() {
        let res = Solution::unique_occurrences(vec![1, 2, 2, 1, 1, 3]);
        assert!(res);

        let res = Solution::unique_occurrences(vec![1, 2]);
        assert!(!res);
    }
}
