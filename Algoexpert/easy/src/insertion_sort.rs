struct InsertionSort {}

impl InsertionSort {
    fn sort(n: Vec<i32>) -> Vec<i32> {
        let mut target = n;

        for i in 0..target.len() {
            for j in i..target.len() {
                if target[i] > target[j] {
                    target.swap(i, j);
                }
            }
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use crate::insertion_sort::InsertionSort;

    #[test]
    fn test() {
        let v = vec![8, 5, 2, 9, 5, 6, 3];
        assert_eq!(InsertionSort::sort(v), vec![2, 3, 5, 5, 6, 8, 9])
    }
}