struct Solution {}

impl Solution {
    pub fn missing_element(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() < k as usize {
            return -1;
        }
        use std::cmp::Ordering;

        let mut missing = vec![];

        // get missing numbers
        for i in 0..k {}

        let mut start: i32 = 0;
        let mut end: i32 = (nums.len() - 1) as i32;

        print!("foo");
        1

        // while start <= end {
        //     let middle = start + (end - start) / 2;
        //
        //     match nums[middle as usize].cmp(&target) {
        //         Ordering::Less => {
        //             start = middle + 1;
        //         }
        //         Ordering::Equal => match do_right_or_left_search {
        //             true => {
        //                 if (middle == start) || nums[(middle - 1) as usize] != target {
        //                     return middle;
        //                 }
        //
        //                 end = middle - 1;
        //             }
        //             false => {
        //                 if (middle == end) || nums[(middle + 1) as usize] != target {
        //                     return middle;
        //                 }
        //
        //                 start = middle + 1;
        //             }
        //         },
        //         Ordering::Greater => {
        //             end = middle - 1;
        //         }
        //     }
        // }
    }
}

#[cfg(test)]
mod test {
    use crate::missing_element::Solution;

    #[test]
    fn test() {
        assert_eq!(5, Solution::missing_element(vec![4, 7, 9, 10], 1));
    }
}
