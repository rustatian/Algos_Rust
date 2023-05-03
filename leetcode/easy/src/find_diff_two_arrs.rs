struct Solution{}

impl Solution {
    pub fn find_difference(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<Vec<i32>> {
        nums1.sort_unstable();
        nums1.dedup();
        nums2.sort_unstable();
        nums2.dedup();

        let mut start1 = 0;
        let mut start2 = 0;

        let mut ans1 = vec![];
        let mut ans2 = vec![];

        loop {
            if start1 >= nums1.len() {
                while start2 < nums2.len() {
                    ans2.push(nums2[start2]);
                    start2+=1;
                }

                return vec![ans1, ans2];
            }

            if start2 >= nums2.len() {
                while start1 < nums1.len() {
                    ans1.push(nums1[start1]);
                    start1+=1;
                }

                return vec![ans1, ans2];
            }

            let v1 = nums1[start1];
            let v2 = nums2[start2];

            if v1 < v2 {
                ans1.push(v1);
                start1+=1;
                continue;
            }

            if v2 < v1 {
                ans2.push(v2);
                start2+=1;
                continue;
            }

            start1+=1;
            start2+=1;
            continue;
        }
    }
}

mod tests {
    use crate::find_diff_two_arrs::Solution;

    #[test]
    fn test(){
        assert_eq!(vec![vec![1,3], vec![4,6]], Solution::find_difference(vec![1,2,3], vec![2,4,6]));
    }
}
