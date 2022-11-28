use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn smallest_common_element(mut mat: Vec<Vec<i32>>) -> i32 {
        if mat.len() == 1 {
            return -1;
        }

        mat.sort_unstable();

        let mut ans = i32::MAX;

        for el in &mat[0] {
            for m in &mat[1..] {
                let mut low = 0;
                let mut high = mat[0].len();

                while low < high {
                    let middle = low + (high - low) / 2;

                    // println!("middle: {}", m[middle]);
                    // println!("el: {}", el);

                    match el.cmp(&m[middle]) {
                        Ordering::Less => {
                            high = middle;
                            ans = i32::MAX;
                        }
                        Ordering::Equal => {
                            if *el < ans {
                                ans = *el;
                            }
                            break;
                        }
                        Ordering::Greater => {
                            low = middle + 1;
                            ans = i32::MAX;
                        }
                    }
                }
            }
        }

        ans
    }
}

mod tests {
    use crate::find_smallest_common::Solution;

    #[test]
    fn test() {
        assert_eq!(
            382,
            Solution::smallest_common_element(vec![
                vec![382, 3356, 3430, 3714, 4005],
                vec![382, 680, 2057, 5013, 9721],
                vec![382, 4079, 5625, 6368, 9953],
                vec![382, 1042, 6976, 7059, 9101],
                vec![382, 3461, 7213, 8155, 9208]
            ])
        );
        assert_eq!(
            6,
            Solution::smallest_common_element(
                vec![vec![5, 6], vec![5, 6], vec![5, 6], vec![4, 6],]
            )
        );
        assert_eq!(
            5,
            Solution::smallest_common_element(vec![
                vec![1, 2, 3, 4, 5],
                vec![2, 4, 5, 8, 10],
                vec![3, 5, 7, 9, 11],
                vec![1, 3, 5, 7, 9]
            ])
        );
        assert_eq!(
            2,
            Solution::smallest_common_element(vec![vec![1, 2, 3], vec![2, 3, 4], vec![2, 3, 5]])
        );
    }
}
