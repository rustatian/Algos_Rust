struct Solution;

impl Solution {
    pub fn is_good(mut nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return false;
        }

        if nums.len() == 2 {
            if nums[0] == 1 && nums[1] == 1 {
                return true;
            }

            return false;
        }

        nums.sort_unstable();

        if (nums.len() - 1) as i32 != nums[nums.len() - 1] {
            return false;
        }

        let mut idx = 1;
        let mut prev = nums[0];

        while idx < nums.len() - 1 {
            if nums[idx] != prev + 1 {
                return false;
            }

            prev = nums[idx];
            idx += 1;
        }

        if nums[nums.len() - 1] != nums[nums.len() - 2] {
            return false;
        }

        true
    }
}
