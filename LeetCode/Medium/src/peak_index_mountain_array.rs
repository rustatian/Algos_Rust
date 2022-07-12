struct Solution {}

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut low = 0;
        let mut high = arr.len() - 1;

        while low < high {
            let middle = low + (high - low) / 2;

            if arr[middle] < arr[middle + 1] {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        return low as i32;
    }
}

mod tests {
    use super::Solution;

    #[test]
    fn test() {
        assert_eq!(1, Solution::peak_index_in_mountain_array(vec![0, 1, 0]));
    }
}
