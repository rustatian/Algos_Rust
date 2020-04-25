struct Solution {}

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        // GREEDY
        let mut l = nums.len() - 1;

        let mut idx:i32 = nums.len() as i32 -1;

        loop {
            if idx < 0 {
                break;
            }
            if idx + nums[idx as usize] >= l as i32 {
                l = idx as usize;
            }
            idx -=1;
        }

        l == 0
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::can_jump(vec![2, 0]), true);
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
}