struct Solution {}

impl Solution {
    pub fn longest_common_subsequence(mut text1: String, mut text2: String) -> i32 {
        let mut t1: Vec<_> = text1.chars().collect();
        t1.sort_unstable();

        let mut t2: Vec<_> = text2.chars().collect();
        t2.sort_unstable();

        let smallest_len = if t1.len() < t2.len() {
            t1.len()
        } else {
            t2.len()
        };

        let mut count = 0;

        for idx in 0..smallest_len {
            if t1[idx] == t2[idx] {
                count+=1;
            }

        }

        count
    }
}

mod tests {
    use crate::longest_common_subsequence::Solution;

    #[test]
    fn test(){
        assert_eq!(3, Solution::longest_common_subsequence(String::from("abcde"), String::from("ace")));
        assert_eq!(3, Solution::longest_common_subsequence(String::from("abc"), String::from("abc")));
        assert_eq!(0, Solution::longest_common_subsequence(String::from("abc"), String::from("def")));
    }
}
