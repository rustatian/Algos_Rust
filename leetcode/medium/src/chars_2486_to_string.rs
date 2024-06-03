struct Solution {}

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let mut idx = 0;
        let mut pref = 0;

        loop {
            if idx >= s.len() {
                break;
            }
            if pref >= t.len() {
                break;
            }

            if s[idx] == t[pref] {
                pref += 1;
            }

            idx += 1;
        }

        (t.len() - pref) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::chars_2486_to_string::Solution;

    #[test]
    fn test() {
        assert_eq!(
            4,
            Solution::append_characters(String::from("coaching"), String::from("coding"))
        );
        assert_eq!(
            0,
            Solution::append_characters(String::from("abcde"), String::from("a"))
        );
        assert_eq!(
            5,
            Solution::append_characters(String::from("z"), String::from("abcde"))
        );
    }
}
