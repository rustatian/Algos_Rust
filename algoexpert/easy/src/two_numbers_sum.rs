use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn two_numbers_sum(nums: Vec<i32>, target: i32) -> Option<[i32; 2]> {
        let mut m = HashMap::new();

        for num in nums {
            let possible_res = target - num;
            if m.contains_key(&possible_res) {
                return Some([num, possible_res]);
            } else {
                m.insert(num, true);
            }
        }

        None
    }
}

#[test]
fn tests() {
    assert_eq!(
        Solution::two_numbers_sum(vec![3, 5, -4, 8, 11, 1, -1, 6], 10),
        Some([-1, 11])
    );
}
