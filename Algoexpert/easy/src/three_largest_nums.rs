struct ThreeLargestNums {}

impl ThreeLargestNums {
    fn find_three_largest_nums(n: Vec<i32>) -> [i32; 3] {
        let mut res = [std::i32::MIN, std::i32::MIN, std::i32::MIN];

        for num in n.iter() {
            if *num > res[2] {
                res[0] = res[1];
                res[1] = res[2];
                res[2] = *num;
            } else if *num > res[1] {
                res[0] = res[1];
                res[1] = *num;
            } else if *num > res[0] {
                res[0] = *num;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use crate::three_largest_nums::ThreeLargestNums;

    #[test]
    fn test() {
        let v = vec![141, 1, 17, -7, -17, -27, 18, 541, 8, 7, 7];
        let a = [18, 141, 541];
        assert_eq!(ThreeLargestNums::find_three_largest_nums(v), a);
    }
}
