use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        let mut ht:HashSet<i32> = HashSet::from_iter(nums);

        1
    }
}

mod tests {
    use crate::degree_on_an_array::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
    }
}
