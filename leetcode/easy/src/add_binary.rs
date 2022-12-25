struct Solution{}

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let a1 = isize::from_str_radix(&a, 2).unwrap();
        let a2 = isize::from_str_radix(&b, 2).unwrap();

        let res = format!("{:b}", a1+a2);

        res
    }
}

mod tests {
    use crate::add_binary::Solution;

    #[test]
    fn test(){
        assert_eq!(String::from("100"), Solution::add_binary("11".into(), "1".into()));
    }
}