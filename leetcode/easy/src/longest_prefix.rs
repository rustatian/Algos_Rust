/*
Write a function to find the longest common prefix string amongst an array of strings.

If there is no common prefix, return an empty string "".



Example 1:

Input: strs = ["flower","flow","flight"]
Output: "fl"
Example 2:

Input: strs = ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.
 */

struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 1 {
            return strs[0].to_string();
        }

        for i in 0..strs[0].len() {
            let ch = strs[0].chars().nth(i).unwrap();

            for j in 1..strs.len() {
                if i == strs[j].len() || strs[j].chars().nth(i).unwrap() != ch {
                    return strs[0][..i].to_string();
                }
            }
        }

        strs[0].to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::longest_prefix::Solution;

    #[test]
    fn test() {
        let res = Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ]);
        assert_eq!(res, "fl".to_string());
    }
}
