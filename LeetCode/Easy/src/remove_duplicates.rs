use std::borrow::Borrow;

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut res = 0;
        let mut curr_num = i32::MIN;
        let mut idx = 0;

        loop {
            match nums.get(idx) {
                None => {
                    return res;
                }
                Some(val) => {
                    if *val == curr_num {
                        nums.remove(idx);
                        continue;
                    }

                    curr_num = *val;
                    res += 1;
                    idx += 1;
                }
            }
        }
    }
}

mod tests {
    use crate::remove_duplicates::Solution;

    #[test]
    fn test1() {
        let res = Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]);
        assert_eq!(res, 5);
        let res = Solution::remove_duplicates(&mut vec![-1, 0, 0, 0, 0, 3, 3]);
        assert_eq!(res, 5);
    }
}
