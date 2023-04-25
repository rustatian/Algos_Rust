use std::cmp::Ordering;

struct ThreeNumbersSum {}

impl ThreeNumbersSum {
    fn caclulate(mut v: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        v.sort_unstable();

        for (i, _) in v.iter().enumerate() {
            let mut left = i + 1;
            let mut right = v.len() - 1;
            while left < right {
                let current = v[i] + v[right] + v[left];
                match current.cmp(&target) {
                    Ordering::Greater => {
                        right -= 1;
                    }
                    Ordering::Less => {
                        left += 1;
                    }
                    Ordering::Equal => {
                        result.push(vec![v[i], v[left], v[right]]);
                        right -= 1;
                        left += 1;
                    }
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::three_numbers_sum::ThreeNumbersSum;

    #[test]
    fn test() {
        assert_eq!(
            ThreeNumbersSum::caclulate(vec![12, 3, 1, 2, -6, 5, -8, 6], 0),
            vec![vec![-8, 2, 6], vec![-8, 3, 5], vec![-6, 1, 5]]
        )
    }
}
