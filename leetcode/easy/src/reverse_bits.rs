struct Solution {}

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut ans = 0;
        let mut shift = 31;
        let mut x = x;

        while x != 0 {
            ans += (x & 1) << shift;
            shift -= 1;
            x = x >> 1;
        }

        ans
    }
}
