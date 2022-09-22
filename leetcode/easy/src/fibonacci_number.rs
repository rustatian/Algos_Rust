struct Solution {}

impl Solution {
    pub fn fib(mut n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return 1;
        }

        let mut n1 = 0;
        let mut n2 = 1;
        let mut temp = n1 + n2;

        while n > 0 {
            temp = n2;
            n2 += n1;
            n1 = temp;
            n -= 1;
        }

        n1
    }
}

mod tests {
    use crate::fibonacci_number::Solution;

    #[test]
    fn test() {
        // assert_eq!(1, Solution::fib(2));
        // assert_eq!(2, Solution::fib(3));
        assert_eq!(832040, Solution::fib(30));
    }
}
