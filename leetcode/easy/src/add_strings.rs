/*
Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.

You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.



Example 1:

Input: num1 = "11", num2 = "123"
Output: "134"
Example 2:

Input: num1 = "456", num2 = "77"
Output: "533"
Example 3:

Input: num1 = "0", num2 = "0"
Output: "0"
 */

struct Solution {}

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.into_bytes();
        let num2 = num2.into_bytes();

        let mut res = String::from("");
        let mut carry = 0;

        let mut p1: isize = (num1.len() - 1) as isize;
        let mut p2: isize = (num2.len() - 1) as isize;

        while p1 >= 0 || p2 >= 0 {
            let x1 = {
                if p1 >= 0 {
                    num1.get(p1 as usize).unwrap() - b'0'
                } else {
                    0
                }
            };
            let x2 = {
                if p2 >= 0 {
                    num2.get(p2 as usize).unwrap() - b'0'
                } else {
                    0
                }
            };

            let val = (x1 + x2 + carry) % 10;
            carry = (x1 + x2 + carry) / 10;

            res.push(val.to_string().parse().unwrap());
            p1 -= 1;
            p2 -= 1;
        }

        if carry != 0 {
            res.push(carry.to_string().parse().unwrap());
        }
        res.chars().rev().collect::<String>()
    }
}

mod tests {
    use crate::add_strings::Solution;

    #[test]
    fn test() {
        assert_eq!(
            String::from("108"),
            Solution::add_strings(String::from("99"), String::from("9"))
        )
    }
}
