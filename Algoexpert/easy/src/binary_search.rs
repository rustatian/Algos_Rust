use std::ops::Index;
use std::cmp::Ordering;

struct BinarySearch {}


impl BinarySearch {
    fn binary_search(n: Vec<i32>, target: i32) -> i32 {
        BinarySearch::recursive_helper(n.clone(), target, 0, (n.len() - 1) as i32)
    }
    fn binary_search_iterative(n: Vec<i32>, target: i32) -> i32 {
        BinarySearch::recursive_helper(n.clone(), target, 0, (n.len() - 1) as i32)
    }

    fn recursive_helper(n: Vec<i32>, target: i32, left: i32, right: i32) -> i32 {
        if left > right {
            return -1;
        }

        let index = (left + right) / 2;
        let num = n.index(index as usize);

        match (*num).cmp(&target) {
            Ordering::Greater => {
                BinarySearch::recursive_helper(n, target, left, index - 1)
            }
            Ordering::Equal => {
                index
            }
            _ => {
                BinarySearch::recursive_helper(n, target, index + 1, right)
            }
        }
    }

    fn iterative_helper(n: Vec<i32>, target: i32, left: &mut i32, right: &mut i32) -> i32 {
        while left <= right {
            let index = (*left + *right) / 2;
            let num = n.index(index as usize);

            match (*num).cmp(&target) {
                Ordering::Equal => {
                    return index;
                }
                Ordering::Less => {
                    *right = index - 1;
                    continue;
                }
                _ => {
                    *left = index + 1;
                    continue;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::BinarySearch;

    #[test]
    fn test_recursive() {
        let v = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        assert_eq!(3, BinarySearch::binary_search(v, 33));

        let v = vec![1, 5, 23, 111];
        assert_eq!(-1, BinarySearch::binary_search(v, 122));
    }

    #[test]
    fn test_iterative() {
        let v = vec![0, 1, 21, 33, 45, 45, 61, 71, 72, 73];
        assert_eq!(3, BinarySearch::binary_search_iterative(v, 33));

        let v = vec![1, 5, 23, 111];
        assert_eq!(-1, BinarySearch::binary_search_iterative(v, 122));
    }
}