struct Solution {}

impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        if Solution::sqrt_int(n).pow(2) == n {
            return 1;
        }

        for i in 1..=Solution::sqrt_int(n) {
            if n - i.pow(2) == Solution::sqrt_int(n - i.pow(2)).pow(2) {
                return 2;
            }
        }

        // https://en.wikipedia.org/wiki/Legendre%27s_three-square_theorem
        // n = 4^a(8b + 7)
        // 4^a
        while (n % 4) == 0 {
            n /= 4;
        }

        // 8b + 7
        if n % 8 != 7 {
            return 3;
        }

        // https://en.wikipedia.org/wiki/Lagrange%27s_four-square_theorem
        4
    }

    fn sqrt_int(n: i32) -> i32 {
        (n as f32).sqrt().floor() as i32
    }
}

mod tests {
    use crate::perfect_squares::Solution;

    #[test]
    fn test() {
        assert_eq!(1, Solution::num_squares(4));
        assert_eq!(2, Solution::num_squares(18));
        assert_eq!(3, Solution::num_squares(12));
        assert_eq!(2, Solution::num_squares(13));
    }
}
