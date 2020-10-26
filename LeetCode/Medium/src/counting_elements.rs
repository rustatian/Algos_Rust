// Given an integer array arr, count element x such that x + 1 is also in arr.
//
// If there're duplicates in arr, count them seperately.
//
//
//
// Example 1:
//
// Input: arr = [1,2,3]
// Output: 2
// Explanation: 1 and 2 are counted cause 2 and 3 are in arr.
//
// Example 2:
//
// Input: arr = [1,1,3,3,5,5,7,7]
// Output: 0
// Explanation: No numbers are counted, cause there's no 2, 4, 6, or 8 in arr.
//
// Example 3:
//
// Input: arr = [1,3,2,3,5,0]
// Output: 3
// Explanation: 0, 1 and 2 are counted cause 1, 2 and 3 are in arr.
//
// Example 4:
//
// Input: arr = [1,1,2,2]
// Output: 2
// Explanation: Two 1s are counted cause 2 is in arr.
//
//
//
// Constraints:
//
// 1 <= arr.length <= 1000
// 0 <= arr[i] <= 1000
//
struct Solution {}

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut res = 0;
        let mut m: HashMap<i32, i32> = HashMap::new();

        // indexation operation
        // we need to know how much of each number is in Vector
        for e in arr.iter() {
            m.entry(*e).and_modify(|x| *x += 1).or_insert(1);
        }

        // In the second loop we are iterate over the initial array
        // But for indexes get x + 1
        // for example our initial array was 1 1 2
        // in hashmap it would be represented as [1] (key) --> 2 (times), [2] --> 1
        // loop
        // e = 1
        // get 1 --> e + 1 --> 2 (two is present)
        // get 1 --> e + 1 --> 2
        // get 2 --> e + 1 --> 3 (no such value)
        for e in arr.iter() {
            if let Some(x) = m.get(&(*e + 1)) {
                if *x > 0 {
                    res += 1;
                }
            }
        }

        res
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::count_elements(vec![1, 1, 2]), 2);
    assert_eq!(Solution::count_elements(vec![1, 2, 3]), 2);
    assert_eq!(Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]), 0);
    assert_eq!(Solution::count_elements(vec![1, 3, 2, 3, 5, 0]), 3);
    assert_eq!(Solution::count_elements(vec![1, 1, 2, 2]), 2);
}