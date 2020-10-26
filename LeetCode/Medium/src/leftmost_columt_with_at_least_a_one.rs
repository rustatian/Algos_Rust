struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        use std::borrow::Borrow;

        let mut h: HashMap<i32, i32> = HashMap::new();
        h.insert(0, 1);
        let mut count = 0;
        let mut sum = 0;

        for (i, v) in nums.iter().enumerate() {
            sum += v;

            if let Some(val) = h.get((sum - k).borrow()) {
                count += val;
            }

            if let Some(val) = h.get(&sum) {
                h.insert(sum, val + 1);
            } else {
                h.insert(sum, 1);
            }
        }
        count
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}

