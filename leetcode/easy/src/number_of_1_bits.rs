struct Solution {}

impl Solution {
    pub fn hammingWeight (n: u32) -> i32 {
        let m1: u64 = 0x5555555555555555;
        let m2: u64 = 0x3333333333333333;
        let m4: u64 = 0x0f0f0f0f0f0f0f0f;
        let h01: u64 = 0x0101010101010101;

        let mut nn = n as u64;

        nn -= (nn >> 1) & m1;
        nn = (nn & m2) + ((nn >> 2) & m2);
        nn = nn + (nn >> 4) & m4;
        ((nn * h01) >> 56) as i32
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test(){}
}
