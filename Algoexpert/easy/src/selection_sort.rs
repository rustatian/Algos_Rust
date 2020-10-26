struct SelectionSort {}

impl SelectionSort {
    fn sort(v: Vec<i32>) -> Vec<i32> {
        let mut smallest = i32::MAX;
        let mut index = 0;
        let mut target = v;

        for i in 0..target.len() {
            for j in i + 1..target.len() {
                if target[j] < target[i] && target[j] < smallest {
                    smallest = target[j];
                    index = j;
                }
            }
            if smallest == i32::MAX {
                continue;
            }

            target.swap(i, index);
            smallest = i32::MAX;
        }
        target
    }
}

#[cfg(test)]
mod tests {
    use crate::selection_sort::SelectionSort;

    #[test]
    fn test() {
        assert_eq!(
            SelectionSort::sort(vec![8, 5, 2, 9, 5, 6, 3]),
            vec![2, 3, 5, 5, 6, 8, 9]
        )
    }
}
