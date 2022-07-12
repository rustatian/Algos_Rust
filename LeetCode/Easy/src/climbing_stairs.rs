struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }

        if n == 2 {
            return 2;
        }

        if n == 3 {
            return 3;
        }

        let mut v = vec![0; n as usize + 1];

        v.insert(1, 1);
        v.insert(2, 2);

        for i in 3..n as usize + 1 {
            v[i] = v[i - 1] + v[i - 2];
        }

        v[n as usize]
    }

    pub fn climb_stairs2(n: i32) -> i32 {
        let sqrt5 = f64::sqrt(5.0);
        let phi = (1.0 + sqrt5) / 2.0;
        let psi = (1.0 - sqrt5) / 2.0;

        ((f64::powf(phi, n as f64 + 1.0) - f64::powf(psi, n as f64 + 1.0)) / sqrt5).floor() as i32
    }
}

mod tests {
    use crate::climbing_stairs::Solution;

    #[test]
    fn test() {
        assert_eq!(55, Solution::climb_stairs2(9));
        assert_eq!(2, Solution::climb_stairs(2));
        assert_eq!(3, Solution::climb_stairs(3));
        assert_eq!(5, Solution::climb_stairs(4));
    }
}
