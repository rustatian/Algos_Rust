struct Solution {}

impl Solution {
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        let mut shift = 0;
        let mut mr = m;
        let mut nr = n;

        while mr < nr {
            mr >>= 1;
            nr >>= 1;
            shift += 1;
        }

        mr << shift
    }
}

#[test]
fn tests() {
    assert_eq!(Solution::range_bitwise_and(9, 12), 8);
    assert_eq!(Solution::range_bitwise_and(5, 7), 4);
    assert_eq!(Solution::range_bitwise_and(0, 2), 0);
}