use std::collections::VecDeque;

struct Solution {}

/*
Given a string s, return true if the s can be palindrome after deleting at most one character from it.

Example 1:

Input: s = "aba"
Output: true

Example 2:

Input: s = "abca"
Output: true
Explanation: You could delete the character 'c'.
Example 3:

Input: s = "abc"
Output: false

 */
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        if s.len() < 2 {
            return false;
        }

        let v: VecDeque<char> = s.chars().collect();

        let mut r_ptr = v.len() - 1;
        let mut l_ptr = 0;
        loop {
            if l_ptr > r_ptr {
                return true
            }

            // get r ptr value
            let l_val = v.get(l_ptr).unwrap();

            // get l ptr value
            let r_val = v.get(r_ptr).unwrap();

            // we don't know, should we remove right or left letter
            // so we check both cases
            if l_val != r_val {
                return check(&v, l_ptr, r_ptr - 1) || check(&v, l_ptr + 1, r_ptr);
            }

            l_ptr += 1;
            r_ptr -= 1;
        }
    }
}

fn check(v: &VecDeque<char>, i: usize, j: usize) -> bool {
    let mut ii = i;
    let mut jj = j;
    while ii < jj {
        if v.get(ii) != v.get(jj) {
            return false;
        }
        ii += 1;
        jj -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::palindrome::Solution;

    #[test]
    fn test() {
        let res = Solution::valid_palindrome("abca".into());
        assert!(res);
        let res = Solution::valid_palindrome("deeee".into());
        assert!(res);
    }
}
