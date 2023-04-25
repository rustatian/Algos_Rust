struct Solution {}

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        // count not from 1, but from 0
        let k = k - 1;
        let mut start: isize = 0;
        let mut end: isize = (arr.len() - 1) as isize;
        let mut middle = 0;

        while start <= end {
            middle = start + (end - start) / 2;

            // how many elements are missing
            // [3,10], 2:
            // middle = 0 + (1 - 0) / 2 = 0;
            // num_missing = 3 - 0 - 1 = 2. If no missing, than we should have a 0 at 0th index, but we have 3
            // if num_missing is less that k, than we increasing the start
            // otherwise -> lower the end
            let num_missing = arr[middle as usize] - middle as i32 - 1;
            if num_missing < k {
                start = middle + 1;
            } else {
                end = middle - 1;
            }
        }

        arr[middle as usize] + k - (arr[middle as usize] - start as i32 - 1)
        // at this point we know, that the number is somewhere between
        // arr[start] arr[end], for example, test2, between [7,11].
        // number of missing elements before arr[end] is end - 1 (arr[3] = 7, [1, 2, 3, 4, 5, 6, 7][3] -> 4, 7-4 = 3.
        //
    }
}

mod tests {
    use crate::k_th_missing_positive_number::Solution;

    #[test]
    fn test() {
        // assert_eq!(2, Solution::find_kth_positive(vec![3, 10], 2));
        assert_eq!(9, Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
        assert_eq!(6, Solution::find_kth_positive(vec![1, 2, 3, 4], 2));
    }
}
