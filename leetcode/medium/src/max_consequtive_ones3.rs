struct Solution {}

impl Solution {
    #[allow(clippy::explicit_counter_loop)]
    pub fn longest_ones(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut l = 0;

        for i in 0..nums.len() {
            if nums[i] == 0 {
                k -= 1;
            }

            if k < 0 {
                k += 1 - nums[l];
                l += 1;
            }
        }

        (nums.len() - l) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::max_consequtive_ones3::Solution;

    #[test]
    fn test() {
        assert_eq!(0, Solution::longest_ones(vec![0, 0, 0, 0], 0));
        assert_eq!(
            32,
            Solution::longest_ones(
                vec![
                    1, 0, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1,
                    1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 1
                ],
                9
            )
        );
        assert_eq!(
            4,
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1], 0)
        );
        assert_eq!(3, Solution::longest_ones(vec![0, 0, 1, 1, 1, 0, 0], 0));
        assert_eq!(
            6,
            Solution::longest_ones(vec![1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2)
        );
        assert_eq!(
            10,
            Solution::longest_ones(
                vec![0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1],
                3
            )
        );
    }
}
