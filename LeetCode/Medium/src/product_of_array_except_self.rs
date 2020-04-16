// Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].
//
// Example:
//
// Input:  [1,2,3,4]
// Output: [24,12,8,6]
//
// Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.
//
// Note: Please solve it without division and in O(n).
//
// Follow up:
// Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)

struct Solution {}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let mut pre = 1;

        for (_, n) in nums.iter().enumerate() {
            res.push(pre);
            pre *= *n;
        }

        let mut post = 1;
        for (idx, n) in nums.iter().enumerate().rev() {
            res[idx] *= post;
            post *= *n;
        }

        res
    }

    pub fn product_except_self_divide(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];

        let max: i32 = nums.iter().product();

        for (_, n) in nums.iter().enumerate() {
            res.push(max / n);
        }

        res
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(Solution::product_except_self_divide(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
}