struct Solution {}

// 8m48s

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;

        // num -> count
        let mut hm: HashMap<i32, i32> = HashMap::new();
        let mut res = vec![];

        // build hashmap
        for i in nums1 {
            match hm.entry(i) {
                Entry::Occupied(mut oe) => {
                    (*oe.get_mut()) += 1;
                }
                Entry::Vacant(ve) => {
                    ve.insert(1);
                }
            }
        }

        for i in nums2 {
            match hm.entry(i) {
                Entry::Occupied(mut oe) => {
                    if *oe.get() == 1 {
                        oe.remove();
                    } else {
                        *oe.get_mut() -= 1;
                    }

                    res.push(i);
                }
                Entry::Vacant(_) => {
                    continue;
                }
            }
        }

        res
    }
}

mod tests {
    use crate::intersection_of_two_arrays::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2])
        );

        assert_eq!(
            vec![9, 4],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4])
        );
    }
}
