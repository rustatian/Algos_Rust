// Given an array of strings, group anagrams together.
//
// Example:
//
// Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
// Output:
// [
// ["ate","eat","tea"],
// ["nat","tan"],
// ["bat"]
// ]
// Note:
//
// All inputs will be in lowercase.
// The order of your output does not matter.
struct Solution {}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {

    }
}

#[test]
fn tests() {
    assert_eq!(Solution::group_anagrams(vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()]),
               vec![vec!["ate".to_string(), "eat".to_string(), "tea".to_string()], vec!["nat".to_string(), "tan".to_string()], vec!["bat".to_string()]]);
}