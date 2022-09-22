struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        use std::cmp::Ordering;

        let mut start = 0;
        let mut end = x;
        let mut answer = 0;

        while start <= end {
            let middle = start + (end - start) / 2;

            match middle.checked_mul(middle) {
                Some(val) => match val.cmp(&x) {
                    Ordering::Less => {
                        answer = middle;
                        start = middle + 1;
                    }
                    Ordering::Equal => {
                        return middle;
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

        answer
    }
}

mod tests {
    use crate::sqrt::Solution;

    #[test]
    fn test() {
        assert_eq!(2, Solution::my_sqrt(5));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(1, Solution::my_sqrt(3));
        assert_eq!(1, Solution::my_sqrt(2));
        assert_eq!(1, Solution::my_sqrt(1));
        assert_eq!(0, Solution::my_sqrt(0));

        assert_eq!(2, Solution::my_sqrt(6));
        assert_eq!(2, Solution::my_sqrt(4));
        assert_eq!(2, Solution::my_sqrt(8));
    }
}
