struct Solution {}

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x == 0 {
            return 0;
        }

        if x == 1 {
            return 1;
        }

        let mut left = 0;
        let mut right = x;
        let mut answer = 0;

        while left < right {
            let mid: usize = (left + (right - left) / 2) as usize;

            match (mid * mid).cmp(&(x as usize)) {
                std::cmp::Ordering::Equal => {
                    return mid as i32;
                }

                std::cmp::Ordering::Less => {
                    left = (mid + 1) as i32;
                    if left * left <= x {
                        answer = left;
                    } else {
                        answer = mid as i32;
                    }
                }

                std::cmp::Ordering::Greater => {
                    right = (mid - 1) as i32;
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
        assert_eq!(46339, Solution::my_sqrt(2147395599));
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
