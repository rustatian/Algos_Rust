/*
Given an integer x, return true if x is palindrome integer.

An integer is a palindrome when it reads the same backward as forward.

For example, 121 is a palindrome while 123 is not.


Example 1:

Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
Example 2:

Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
Example 3:

Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
 */
use std::collections::VecDeque;

struct Solution {}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }

        let v: VecDeque<char> = x.to_string().chars().collect();

        if v.len() == 1 {
            return false;
        }

        let mut r_ptr = v.len() - 1;
        let mut l_ptr = 0;
        loop {
            if l_ptr > r_ptr {
                return true;
            }

            // get r ptr value
            let l_val = v.get(l_ptr).unwrap();

            // get l ptr value
            let r_val = v.get(r_ptr).unwrap();

            // we don't know, should we remove right or left letter
            // so we check both cases
            if l_val != r_val {
                return false;
            }

            l_ptr += 1;
            r_ptr -= 1;
        }
    }

    fn is_palindrome_smart(x: i32) -> bool {
        let mut x = x;
        if (x < 0) || (x % 10 == 0 && x != 0) {
            return false;
        }

        let mut reverted = 0;

        while x > reverted {
            reverted = reverted * 10 + x % 10;
            x /= 10;
        }

        x == reverted || x == reverted / 10
    }
}

#[cfg(test)]
mod tests {
    use crate::palindrome_2::Solution;

    #[test]
    fn test() {
        let res = Solution::is_palindrome(10);
        assert!(!res);
    }
}
