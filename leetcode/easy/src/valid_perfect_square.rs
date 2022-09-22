struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        use std::cmp::Ordering;
        if num == 1 {
            return true;
        }

        let mut start = 0;
        let mut end = num - 1;

        while start <= end {
            let middle = start + (end - start) / 2;

            match middle.checked_mul(middle) {
                Some(guess) => match guess.cmp(&num) {
                    Ordering::Less => {
                        start = middle + 1;
                    }
                    Ordering::Equal => {
                        return true;
                    }
                    Ordering::Greater => {
                        end = middle - 1;
                    }
                },
                None => {
                    end = middle - 1;
                }
            }
        }

        false
    }
}

mod tests {
    use crate::valid_perfect_square::Solution;

    #[test]
    fn test() {
        assert!(Solution::is_perfect_square(9));
        assert!(Solution::is_perfect_square(16));
        assert!(!Solution::is_perfect_square(14));
    }
}
