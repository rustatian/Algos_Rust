struct Solution {}

impl Solution {
    pub fn orderly_queue(s: String, k: i32) -> String {
        if k == 1 {
            let ss = s.chars().chain(s.chars()).collect::<Vec<char>>();
            return ss.windows(s.len()).min().unwrap().iter().collect();
        }

        let mut v = s.chars().collect::<Vec<char>>();
        v.sort_unstable();
        v.iter().collect()
    }
}

mod test {
    use crate::orderly_queue::Solution;

    #[test]
    fn test() {
        assert_eq!(
            String::from("acb"),
            Solution::orderly_queue(String::from("cba"), 1)
        );
        assert_eq!(
            String::from("aaabc"),
            Solution::orderly_queue(String::from("baaca"), 3)
        );
    }
}
