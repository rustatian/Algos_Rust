

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
        use std::collections::HashMap;
        // results
        let mut res = vec![];
        // hashmap to collect results
        let mut hash: HashMap<String, Vec<String>> = HashMap::new();
        for s in strs {
            // get the string
            let mut v: Vec<char> = s.chars().collect();
            // sort it
            // eta would be --> aet
            // tea also would be --> aet
            v.sort();
            // rust thing, we need to convert v from Vec<char> to Vec<String>
            let sorted = v.iter().collect::<String>();
            // see, if there are any keys (keep in mind, that for anagrams, sorted key will be the same for all)
            let e = hash.entry(sorted).or_insert_with(|| vec![]);
            // push not initial string
            e.push(s)
        }
        // here we just collect the results
        for v in hash.values() {
            res.push(v.to_owned());
        }
        res
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::group_anagrams(vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string()]),
               vec![vec![
                   "ate".to_string(),
                   "eat".to_string(),
                   "tea".to_string()],
                    vec![
                        "nat".to_string(),
                        "tan".to_string()],
                    vec!["bat".to_string()
                    ]]);
}