struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let first_idx = Solution::subarray_search(&nums, target, true);

        if first_idx == -1 {
            return vec![-1, -1];
        }

        let last_idx = Solution::subarray_search(&nums, target, false);

        return vec![first_idx, last_idx];
    }

    pub fn subarray_search(nums: &Vec<i32>, target: i32, do_right_or_left_search: bool) -> i32 {
        use std::cmp::Ordering;

        let mut start: i32 = 0;
        let mut end: i32 = (nums.len() - 1) as i32;

        while start <= end {
            let middle = start + (end - start) / 2;

            match nums[middle as usize].cmp(&target) {
                Ordering::Less => {
                    start = middle + 1;
                }
                Ordering::Equal => match do_right_or_left_search {
                    true => {
                        if (middle == start) || nums[(middle - 1) as usize] != target {
                            return middle;
                        }

                        end = middle - 1;
                    }
                    false => {
                        if (middle == end) || nums[(middle + 1) as usize] != target {
                            return middle;
                        }

                        start = middle + 1;
                    }
                },
                Ordering::Greater => {
                    end = middle - 1;
                }
            }
        }

        -1
    }
}

mod tests {
    use crate::find_first_and_last_position::Solution;

    #[test]
    fn test() {
        assert_eq!(vec![0, 0], Solution::search_range(vec![1, 3], 1));
        assert_eq!(vec![0, 1], Solution::search_range(vec![2, 2], 2));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![2, 2], 1));
        assert_eq!(vec![-1, -1], Solution::search_range(vec![1], 0));
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
    }
}
