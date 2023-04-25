struct Solution {}

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, mut potions: Vec<i32>, success: i64) -> Vec<i32> {
        potions.sort_unstable();
        let mut res = vec![];
        let biggest = potions[potions.len() - 1] as i64;
        let total = potions.len() as i64;

        for i in 0..spells.len() {
            let min_req = (success as f64 / spells[i] as f64).ceil() as i64;
            // let min_req = min.floor() as i64;
            if min_req > biggest {
                res.push(0 as i32);
                continue;
            } else {
                let tmp = Solution::binary_search(&potions, min_req) as i64;
                res.push((total - tmp) as i32);
            }
        }

        res
    }

    pub fn binary_search(data: &Vec<i32>, val: i64) -> usize {
        use std::cmp::Ordering;

        let mut start = 0;
        let mut end = data.len();

        while start < end {
            let middle = start + (end - start) / 2;

            match data[middle as usize].cmp(&(val as i32)) {
                Ordering::Less => {
                    start = middle + 1;
                }
                _ => {
                    end = middle;
                }
            }
        }

        start
    }
}

mod tests {
    use crate::succesful_pairs::Solution;

    #[test]
    fn test() {
        assert_eq!(
            vec![4, 0, 3],
            Solution::successful_pairs(vec![5, 1, 3], vec![1, 2, 3, 4, 5], 7)
        )
    }
}
