struct Solution {}

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, mut truck_size: i32) -> i32 {
        let mut res = 0;
        box_types.sort_by(|a, b| b[1].cmp(&a[1]));
        for bt in box_types {
            if bt[0] > truck_size {
                return res + truck_size * bt[1];
            } else if truck_size == 0 {
                return res;
            }

            res += bt[0] * bt[1];
            truck_size -= bt[0];
        }

        res
    }
}

mod tests {
    use crate::maximum_units_on_a_truck::Solution;

    #[test]
    fn test() {
        assert_eq!(
            95,
            Solution::maximum_units(
                vec![
                    vec![4, 2],
                    vec![5, 5],
                    vec![4, 1],
                    vec![1, 4],
                    vec![2, 5],
                    vec![1, 3],
                    vec![5, 3],
                    vec![1, 5],
                    vec![5, 5],
                    vec![1, 1]
                ],
                24
            )
        );
        assert_eq!(
            8,
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4)
        );
        assert_eq!(
            91,
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10)
        );
    }
}
