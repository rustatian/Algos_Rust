struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s = s
            .to_lowercase()
            .replace(|c: char| !c.is_alphanumeric(), "")
            .into_bytes();

        if s.is_empty() || s.len() == 1 {
            return true;
        }

        let mut r_ptr = s.len() - 1;
        let mut l_ptr = 0;
        loop {
            if l_ptr > r_ptr {
                return true;
            }

            // get r ptr value
            let l_val = s.get(l_ptr).unwrap();

            // get l ptr value
            let r_val = s.get(r_ptr).unwrap();

            // we don't know, should we remove right or left letter
            // so we check both cases
            if l_val != r_val {
                return false;
            }

            l_ptr += 1;
            r_ptr -= 1;
        }
    }
}

mod tests {
    use crate::valid_palindrome::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(!Solution::is_palindrome("race a car".to_string()));
        assert!(Solution::is_palindrome(" ".to_string()));
        assert!(Solution::is_palindrome("a".to_string()));
        assert!(!Solution::is_palindrome("0P".to_string()));
    }
}
