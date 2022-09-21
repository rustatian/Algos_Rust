struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m - 1;
        let mut p2 = n - 1;

        for p in 0..(m + n) - 1 {
            if p2 < 0 {
                return;
            }
            if p1 >= 0 && nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
        }
    }
}

mod tests {
    use crate::merge_sorted_array::Solution;

    #[test]
    fn test() {
        let mut v = vec![1, 2, 3, 0, 0, 0];
        Solution::merge(&mut v, 3, &mut vec![2, 5, 6], 3);
        assert_eq!(vec![1, 2, 2, 3, 5, 6], v);
    }
}
