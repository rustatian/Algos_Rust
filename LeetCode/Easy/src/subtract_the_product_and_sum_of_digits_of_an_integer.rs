// Given an integer number n, return the difference between the product of its digits and the sum of its digits.
//
//
//
// Example 1:
//
// Input: n = 234
// Output: 15
// Explanation:
// Product of digits = 2 * 3 * 4 = 24
// Sum of digits = 2 + 3 + 4 = 9
// Result = 24 - 9 = 15
//
// Example 2:
//
// Input: n = 4421
// Output: 21
// Explanation:
// Product of digits = 4 * 4 * 2 * 1 = 32
// Sum of digits = 4 + 4 + 2 + 1 = 11
// Result = 32 - 11 = 21
//
//
//
// Constraints:
//
// 1 <= n <= 10^5
pub struct Solution {}

impl Solution {
    pub fn subtract_product_and_sum(n: i32) -> i32 {
        let sum = n
            .to_string()
            .chars()
            .fold(0, |acc, x| acc + x.to_digit(10).unwrap() as i32);
        let product = n
            .to_string()
            .chars()
            .fold(1, |acc, x| acc * x.to_digit(10).unwrap() as i32);
        product - sum
    }
}

#[test]
pub fn solution_tests() {
    assert_eq!(Solution::subtract_product_and_sum(234), 15);
    assert_eq!(Solution::subtract_product_and_sum(4421), 21);
}
