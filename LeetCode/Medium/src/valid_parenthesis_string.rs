// Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid.
// We define the validity of a string by these rules:
//
// Any left parenthesis '(' must have a corresponding right parenthesis ')'.
// Any right parenthesis ')' must have a corresponding left parenthesis '('.
// Left parenthesis '(' must go before the corresponding right parenthesis ')'.
// '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
// An empty string is also valid.
//
// Example 1:
//
// Input: "()"
// Output: True
//
// Example 2:
//
// Input: "(*)"
// Output: True
//
// Example 3:
//
// Input: "(*))"
// Output: True
//
// Note:
//
// The string size will be in the range [1, 100].
//


struct Solution {}

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left_ballance = 0;
        let mut right_balance = 0;

        for ch in s.chars() {
            if ch == '(' {
                left_ballance += 1;
            } else {
                left_ballance -= 1;
            }

            if ch == '(' || ch == '*' {
                right_balance += 1;
            } else {
                right_balance -= 1;
            }

            if right_balance < 0 {
                break;
            }

            if left_ballance < 0 {
                left_ballance = 0;
            }
        }

        left_ballance == 0
    }
}


#[test]
fn tests() {
    assert_eq!(Solution::check_valid_string("(())((())()()(*)(*()(())())())()()((()())((()))(*".to_string()), false);
    assert_eq!(Solution::check_valid_string("(((******))".to_string()), true);
    assert_eq!(Solution::check_valid_string(")(".to_string()), false);
    assert_eq!(Solution::check_valid_string("()".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}