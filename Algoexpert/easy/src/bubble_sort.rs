struct BubbleSort {}

impl BubbleSort {
    fn sort(arr: Vec<i32>) -> Vec<i32> {
        let mut is_sorted = false;
        let mut target = arr;

        while !is_sorted {
            is_sorted = true;
            for i in 0..target.len() - 1 {
                if target[i] > target[i + 1] {
                    is_sorted = false;
                    target.swap(i + 1, i);
                }
            }
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort::BubbleSort;

    #[test]
    fn test() {
        let arr = vec![8, 5, 2, 9, 5, 6, 3];
        assert_eq!(BubbleSort::sort(arr), vec![2, 3, 5, 5, 6, 8, 9])
    }
}