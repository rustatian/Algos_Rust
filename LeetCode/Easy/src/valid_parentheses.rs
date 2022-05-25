struct Solution {}

/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.


Example 1:

Input: s = "()"
Output: true
Example 2:

Input: s = "()[]{}"
Output: true
Example 3:

Input: s = "(]"
Output: false
 */

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // odd
        if s.len() % 2 != 0 {
            return false;
        }

        let mut v: Vec<char> = Vec::new();

        for ch in s.chars() {
            match ch {
                '(' | '{' | '[' => {
                    v.push(ch);
                }
                ')' => match v.last() {
                    Some(val) => {
                        if *val == '(' {
                            v.pop();
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                '}' => match v.last() {
                    Some(val) => {
                        if *val == '{' {
                            v.pop();
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                ']' => match v.last() {
                    Some(val) => {
                        if *val == '[' {
                            v.pop();
                        } else {
                            return false;
                        }
                    }
                    None => {
                        return false;
                    }
                },
                _ => {}
            }
        }

        v.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use crate::valid_parentheses::Solution;

    #[test]
    fn test() {
        let res = Solution::is_valid("{[]}".into());
        assert!(res);
    }
}
