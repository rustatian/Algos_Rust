// Given two strings S and T, return if they are equal when both are typed into empty text editors.
// # means a backspace character.
//
// Example 1:
//
// Input: S = "ab#c", T = "ad#c"
// Output: true
// Explanation: Both S and T become "ac".
//
// Example 2:
//
// Input: S = "ab##", T = "c#d#"
// Output: true
// Explanation: Both S and T become "".
//
// Example 3:
//
// Input: S = "a##c", T = "#a#c"
// Output: true
// Explanation: Both S and T become "c".
//
// Example 4:
//
// Input: S = "a#c", T = "b"
// Output: false
// Explanation: S becomes "c" while T becomes "b".
//
// Note:
//
// 1 <= S.length <= 200
// 1 <= T.length <= 200
// S and T only contain lowercase letters and '#' characters.
//
// Follow up:
//
// Can you solve it in O(N) time and O(1) space?

struct Solution {}

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        Solution::get_string(s) == Solution::get_string(t)
    }

    fn get_string(s: String) -> String {
        let mut ret = String::new();
        for c in s.chars() {
            if c == '#' {
                if ret.is_empty() {
                    continue;
                }
                ret.pop();
            } else {
                ret.push(c);
            }
        }
        ret
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()), true);
    assert_eq!(Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()), true);
    assert_eq!(Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()), true);
    assert_eq!(Solution::backspace_compare("a#c".to_string(), "b".to_string()), false);
    assert_eq!(Solution::backspace_compare("y#fo##f".to_string(), "y#f#o##f".to_string()), true);
    assert_eq!(Solution::backspace_compare("y#fo##f".to_string(), "y#f#o##f".to_string()), true);
}